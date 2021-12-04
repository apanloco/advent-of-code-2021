use crate::error;

struct NavigationResult {
    horizontal_position: u64,
    depth: u64,
    aim: u64,
}

impl NavigationResult {
    pub fn sum(&self) -> u64 {
        self.horizontal_position * self.depth
    }
}

#[derive(PartialEq, Debug)]
enum Command {
    Forward(u64),
    Up(u64),
    Down(u64),
}

impl std::str::FromStr for Command {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(' ').collect();
        if tokens.len() != 2 {
            return Err(error::Error::Parse(format!("invalid command: {}", s)));
        }
        let command = tokens[0];
        let number: u64 = tokens[1].parse()?;
        match command.to_lowercase().as_ref() {
            "forward" => Ok(Command::Forward(number)),
            "up" => Ok(Command::Up(number)),
            "down" => Ok(Command::Down(number)),
            _ => { Err(error::Error::Parse(format!("invalid command: {}", s))) }
        }
    }
}

fn parse_commands(s: &str) -> Result<Vec<Command>, error::Error> {
    let mut commands: Vec<Command> = Vec::new();
    for line in s.lines() {
        let line = line.trim_end().trim_start();
        if line.is_empty() {
            continue;
        }
        commands.push(line.parse()?);
    }
    Ok(commands)
}

fn navigate(commands: &Vec<Command>) -> NavigationResult {
    let mut res = NavigationResult {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };

    for command in commands {
        match command {
            Command::Forward(v) => { res.horizontal_position += v }
            Command::Up(v) => { res.depth -= v }
            Command::Down(v) => { res.depth += v }
        }
    }

    res
}

fn navigate_aim(commands: &Vec<Command>) -> NavigationResult {
    let mut res = NavigationResult {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };

    for command in commands {
        match command {
            Command::Forward(v) => {
                res.horizontal_position += v;
                res.depth += res.aim * v
            }
            Command::Up(v) => { res.aim -= v }
            Command::Down(v) => { res.aim += v }
        }
    }

    res
}

#[test]
fn test_from_string() -> Result<(), error::Error> {
    let input = r#"

  forward 5
down 6
up 31

    "#;

    let commands: Vec<Command> = parse_commands(input)?;
    assert_eq!(commands.len(), 3);
    assert_eq!(commands[0], Command::Forward(5));
    assert_eq!(commands[1], Command::Down(6));
    assert_eq!(commands[2], Command::Up(31));
    Ok(())
}

#[test]
fn test_navigate() -> Result<(), error::Error> {
    let input = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
    "#;

    let commands: Vec<Command> = parse_commands(input)?;

    let navres = navigate(&commands);
    assert_eq!(navres.horizontal_position, 15);
    assert_eq!(navres.depth, 10);
    assert_eq!(navres.sum(), 150);

    let navres = navigate_aim(&commands);
    assert_eq!(navres.horizontal_position, 15);
    assert_eq!(navres.depth, 60);
    assert_eq!(navres.sum(), 900);

    Ok(())
}

#[test]
fn test_navigate_input() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("input_day2")?;
    let commands: Vec<Command> = parse_commands(&input)?;

    let navres = navigate(&commands);
    assert_eq!(navres.horizontal_position, 1967);
    assert_eq!(navres.depth, 1031);
    assert_eq!(navres.sum(), 2027977);

    let navres = navigate_aim(&commands);
    assert_eq!(navres.horizontal_position, 1967);
    assert_eq!(navres.depth, 967791);
    assert_eq!(navres.sum(), 1903644897);
    Ok(())
}
