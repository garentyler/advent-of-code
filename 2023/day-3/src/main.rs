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

    let mut gears: Vec<(Number, Number)> = vec![];

    for symbol in &symbols {
        if symbol.value != '*' {
            continue;
        }

        let mut adjacent_numbers = vec![];

        for number in &numbers {
            if symbol.is_adjacent(number) {
                adjacent_numbers.push(number);
            }
        }

        if adjacent_numbers.len() == 2 {
            gears.push((*adjacent_numbers[0], *adjacent_numbers[1]));
        }
    }

    let mut gear_ratios = gears.into_iter().map(|(n1, n2)| n1.value * n2.value);
    let sum: usize = gear_ratios.sum();

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
                symbols.push(Symbol {
                    value: c,
                    line_num,
                    index: i,
                });
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
    pub value: char,
    pub line_num: usize,
    pub index: usize,
}
impl Symbol {
    pub fn is_adjacent(&self, number: &Number) -> bool {
        number.is_adjacent(self)
    }
}
