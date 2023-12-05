fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    let mut cards: Vec<(usize, Card)> = vec![];

    while let Ok(_chars_read) = stdin.read_line(&mut buffer) {
        if buffer == "" {
            break;
        } else {
            print!("read line: {buffer}");
        }

        let mut card = Card::parse(&buffer).unwrap();
        cards.push((1, card));

        buffer = String::new();
    }
    print!("\n");

    // Loop through and duplicate all the cards.
    let mut i = 0;
    while i < cards.len() {
        let count = cards[i].0;
        let matching_numbers_count = cards[i].1.matching_numbers().len();

        let start = i + 1;
        let end = i + 1 + matching_numbers_count;

        for j in start..end {
            cards[j].0 += count;
        }

        i += 1;
    }

    // Loop through the duplicated cards and sum the score.
    let mut point_sum = 0;
    let mut count_sum = 0;
    for (count, card) in &cards {
        println!("{card:?}\n\tcount: {}, points: {}", count, card.points(),);
        count_sum += count;
        point_sum += card.points();
    }
    println!("point sum: {point_sum}");
    println!("count sum: {count_sum}");

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
        match self.matching_numbers().len() {
            0 => 0,
            n => 2usize.pow(n as u32 - 1),
        }
    }
    pub fn matching_numbers(&self) -> Vec<usize> {
        self.actual_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .map(|&n| n)
            .collect()
    }
}
