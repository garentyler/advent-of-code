use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    let mut games: Vec<Game> = vec![];

    while let Ok(_chars_read) = stdin.read_line(&mut buffer) {
        if buffer == "" {
            break;
        } else {
            print!("read line: {buffer}");
        }

        let game = Game::parse(&buffer).unwrap();
        println!("\t{:?}", game);
        games.push(game);

        buffer = String::new();
    }

    print!("\n");

    let possible_games: Vec<usize> = games
        .iter()
        .filter(|game| {
            game.handfuls.iter().all(|handful| {
                handful.reds() <= 12 && handful.greens() <= 13 && handful.blues() <= 14
            })
        })
        .map(|game| game.id)
        .collect();

    println!("possible games: {:?}", possible_games);
    let sum: usize = possible_games.iter().sum();
    println!("sum: {sum}");

    Ok(())
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Cube {
    Red,
    Green,
    Blue,
}
impl Cube {
    pub fn parse(input: &str) -> Result<(Cube, &str), &str> {
        if input.starts_with("red") {
            Ok((Cube::Red, &input[3..]))
        } else if input.starts_with("green") {
            Ok((Cube::Green, &input[5..]))
        } else if input.starts_with("blue") {
            Ok((Cube::Blue, &input[4..]))
        } else {
            Err(input)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Handful {
    pub cubes: HashMap<Cube, usize>,
}
impl Handful {
    pub fn parse(input: &str) -> Result<Handful, &str> {
        // Vec of "3 red" and "30 blue", etc.
        let cube_strs: Vec<&str> = input.split(",").map(|s| s.trim()).collect();
        let mut cubes = HashMap::new();

        for s in cube_strs {
            let s: Vec<&str> = s.split_whitespace().collect();
            let count: usize = s[0].parse().map_err(|_| "")?;
            let cube = Cube::parse(s[1]).map_err(|_| "")?.0;

            if let Some(c) = cubes.get(&cube) {
                cubes.insert(cube, c + count);
            } else {
                cubes.insert(cube, count);
            }
        }

        Ok(Handful { cubes })
    }
    pub fn total(&self) -> usize {
        self.reds() + self.greens() + self.blues()
    }
    pub fn reds(&self) -> usize {
        *self.cubes.get(&Cube::Red).unwrap_or(&0)
    }
    pub fn greens(&self) -> usize {
        *self.cubes.get(&Cube::Green).unwrap_or(&0)
    }
    pub fn blues(&self) -> usize {
        *self.cubes.get(&Cube::Blue).unwrap_or(&0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Game {
    pub id: usize,
    pub handfuls: Vec<Handful>,
}
impl Game {
    pub fn parse(mut input: &str) -> Result<Game, &str> {
        input = input.strip_prefix("Game ").ok_or(input)?;
        let s: Vec<&str> = input.split(":").collect();

        let id: usize = s[0].parse().map_err(|_| input)?;

        let mut handfuls: Vec<_> = s[1].split(";").map(|s| Handful::parse(s)).collect();
        if handfuls.iter().any(|h| h.is_err()) {
            return Err(input);
        }
        let handfuls: Vec<Handful> = handfuls.into_iter().map(|h| h.unwrap()).collect();

        Ok(Game { id, handfuls })
    }
}
