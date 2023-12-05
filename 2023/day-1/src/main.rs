fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    let mut numbers: Vec<usize> = vec![];

    while let Ok(_chars_read) = stdin.read_line(&mut buffer) {
        if buffer == "" {
            break;
        } else {
            print!("read line: {buffer}");
        }

        let first = get_first_digit(&buffer).unwrap();
        let last = get_last_digit(&buffer).unwrap();

        let number: usize = format!("{first}{last}").parse().unwrap();
        numbers.push(number);

        println!("\t{number}");

        // Empty the string.
        buffer = String::new();
    }

    println!("\nnumbers: {numbers:?}");
    let sum: usize = numbers.iter().sum();
    println!("sum: {sum}");

    Ok(())
}

fn get_first_digit(s: &str) -> Option<usize> {
    let mut chars = s.chars();

    while !chars.as_str().is_empty() {
        let s = chars.as_str();

        if s.starts_with("0") || s.starts_with("zero") {
            return Some(0);
        } else if s.starts_with("1") || s.starts_with("one") {
            return Some(1);
        } else if s.starts_with("2") || s.starts_with("two") {
            return Some(2);
        } else if s.starts_with("3") || s.starts_with("three") {
            return Some(3);
        } else if s.starts_with("4") || s.starts_with("four") {
            return Some(4);
        } else if s.starts_with("5") || s.starts_with("five") {
            return Some(5);
        } else if s.starts_with("6") || s.starts_with("six") {
            return Some(6);
        } else if s.starts_with("7") || s.starts_with("seven") {
            return Some(7);
        } else if s.starts_with("8") || s.starts_with("eight") {
            return Some(8);
        } else if s.starts_with("9") || s.starts_with("nine") {
            return Some(9);
        }

        let _ = chars.next();
    }

    None
}

fn get_last_digit(s: &str) -> Option<usize> {
    let mut chars = s.chars();

    while !chars.as_str().is_empty() {
        let s = chars.as_str();

        if s.ends_with("0") || s.ends_with("zero") {
            return Some(0);
        } else if s.ends_with("1") || s.ends_with("one") {
            return Some(1);
        } else if s.ends_with("2") || s.ends_with("two") {
            return Some(2);
        } else if s.ends_with("3") || s.ends_with("three") {
            return Some(3);
        } else if s.ends_with("4") || s.ends_with("four") {
            return Some(4);
        } else if s.ends_with("5") || s.ends_with("five") {
            return Some(5);
        } else if s.ends_with("6") || s.ends_with("six") {
            return Some(6);
        } else if s.ends_with("7") || s.ends_with("seven") {
            return Some(7);
        } else if s.ends_with("8") || s.ends_with("eight") {
            return Some(8);
        } else if s.ends_with("9") || s.ends_with("nine") {
            return Some(9);
        }

        let _ = chars.next_back();
    }

    None
}
