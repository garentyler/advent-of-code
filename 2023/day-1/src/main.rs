fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    let mut numbers: Vec<usize> = vec![];

    while let Ok(_chars_read) = stdin.read_line(&mut buffer) {
        if buffer == "" {
            break;
        }

        let digits: Vec<char> = buffer.chars().filter(|c| c.is_numeric()).collect();
        let first: char;
        let last: char;

        match digits.len() {
            0 => panic!("line has no digits"),
            _ => {
                first = *digits.first().unwrap();
                last = *digits.last().unwrap();
            }
        }

        let number: usize = format!("{first}{last}").parse().unwrap();
        numbers.push(number);

        // Empty the string.
        buffer = String::new();
    }

    println!("numbers: {numbers:?}");
    let sum: usize = numbers.iter().sum();
    println!("sum: {sum}");

    Ok(())
}
