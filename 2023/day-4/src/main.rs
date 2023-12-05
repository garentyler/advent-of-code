fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    let mut cards: Vec<Card> = vec![];

    while let Ok(_chars_read) = stdin.read_line(&mut buffer) {
        if buffer == "" {
            break;
        } else {
            print!("read line: {buffer}");
        }

        let mut card = Card::parse(&buffer).unwrap();
        cards.push(card);

        buffer = String::new();
    }
    print!("\n");

    let mut sum = 0;
    for card in &cards {
        println!("{card:?}\n\tpoints: {}", card.points());
        sum += card.points();
    }
    println!("sum: {sum}");

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct Card {
    pub id: usize,
    pub winning_numbers: Vec<usize>,
    pub actual_numbers: Vec<usize>,
}
impl Card {
    pub fn parse(mut input: &str) -> Result<Card, ()> {
        input = input.strip_prefix("Card ").ok_or(())?;
        let s: Vec<&str> = input.split(":").collect();

        let id: usize = s[0].trim().parse().map_err(|_| ())?;
        let nums: Vec<&str> = s[1].split("|").collect();

        let w = nums[0]
            .trim()
            .split_whitespace()
            .map(|n| n.trim())
            .filter(|&n| n != "")
            .map(|n| n.parse::<usize>());
        let a = nums[1]
            .trim()
            .split_whitespace()
            .map(|n| n.trim())
            .filter(|&n| n != "")
            .map(|n| n.parse::<usize>());

        let mut winning_numbers = vec![];
        let mut actual_numbers = vec![];

        for n in w {
            if let Ok(n) = n {
                winning_numbers.push(n);
            } else {
                println!("Failed to parse number {n:?}");
                return Err(());
            }
        }
        for n in a {
            if let Ok(n) = n {
                actual_numbers.push(n);
            } else {
                println!("Failed to parse number {n:?}");
                return Err(());
            }
        }

        Ok(Card {
            id,
            winning_numbers,
            actual_numbers,
        })
    }
    pub fn points(&self) -> usize {
        match self.actual_numbers_winning().len() {
            0 => 0,
            n => 2usize.pow(n as u32 - 1),
        }
    }
    pub fn actual_numbers_winning(&self) -> Vec<usize> {
        self.actual_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .map(|&n| n)
            .collect()
    }
}
