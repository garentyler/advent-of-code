use std::ops::Range;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    let mut line_num = 0;
    while let Ok(_chars_read) = stdin.read_line(&mut buffer) {
        if buffer == "" {
            break;
        } else {
            print!("read line: {buffer}");
        }

        let mut line = Line::parse(line_num, &buffer).unwrap();
        numbers.append(&mut line.numbers);
        symbols.append(&mut line.symbols);

        buffer = String::new();
        line_num += 1;
    }
    print!("\n");

    println!("numbers: {numbers:?}");
    println!("symbols: {symbols:?}");

    numbers.retain(|number| {
        for symbol in &symbols {
            if number.is_adjacent(symbol) {
                return true;
            }
        }
        false
    });

    println!("adjacents: {numbers:?}");

    let mut sum = 0usize;
    for number in &numbers {
        sum += number.value;
    }

    println!("sum: {sum}");

    Ok(())
}

#[derive(Clone, Debug)]
struct Schematic {
    pub lines: Vec<Line>,
}

#[derive(Clone, Debug)]
struct Line {
    pub line_num: usize,
    pub numbers: Vec<Number>,
    pub symbols: Vec<Symbol>,
}
impl Line {
    pub fn parse(line_num: usize, line: &str) -> Result<Line, ()> {
        let mut number_start: Option<usize> = None;
        let mut numbers: Vec<Number> = vec![];
        let mut symbols: Vec<Symbol> = vec![];

        let mut i = 0;
        for c in line.chars() {
            if c.is_numeric() {
                if number_start.is_none() {
                    number_start = Some(i);
                }
            } else if let Some(n) = number_start {
                numbers.push(Number {
                    line_num,
                    value: line[n..i].parse().map_err(|_| ())?,
                    start: n,
                    num_digits: i - n,
                });
                number_start = None;
            }

            if c.is_ascii_punctuation() && c != '.' {
                symbols.push(Symbol { line_num, index: i });
            }

            i += 1;
        }

        Ok(Line {
            line_num,
            numbers,
            symbols,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Number {
    pub line_num: usize,
    pub value: usize,
    pub start: usize,
    pub num_digits: usize,
}
impl Number {
    pub fn is_adjacent(&self, symbol: &Symbol) -> bool {
        if self.line_num == symbol.line_num
            || self.line_num + 1 == symbol.line_num
            || self.line_num == symbol.line_num + 1
        {
            let start = if self.start == 0 { 0 } else { self.start - 1 };
            let end = self.start + self.num_digits;

            start <= symbol.index && symbol.index <= end
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Symbol {
    pub line_num: usize,
    pub index: usize,
}
