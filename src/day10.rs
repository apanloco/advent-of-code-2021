use crate::error;

#[derive(PartialEq, Debug)]
pub enum ParserError {
    InvalidClosingChar(char),
    InvalidChar(char),
}

pub struct Parser {
    pub stack: Vec<char>,
}

impl Parser {
    pub fn default() -> Self {
        Parser { stack: vec![] }
    }

    fn is_open(c: char) -> bool {
        c == '(' || c == '[' || c == '{' || c == '<'
    }

    fn is_close(c: char) -> bool {
        c == ')' || c == ']' || c == '}' || c == '>'
    }

    fn matches(open: char, close: char) -> bool {
        (open == '(' && close == ')') || (open == '[' && close == ']') || (open == '{' && close == '}') || (open == '<' && close == '>')
    }

    pub fn feed(&mut self, input: char) -> Result<(), ParserError> {
        if Parser::is_open(input) {
            self.stack.push(input);
            Ok(())
        } else if Parser::is_close(input) {
            match self.stack.pop() {
                Some(open) => {
                    if Parser::matches(open, input) {
                        Ok(())
                    } else {
                        self.stack.push(open);
                        Err(ParserError::InvalidClosingChar(input))
                    }
                }
                None => Err(ParserError::InvalidClosingChar(input)),
            }
        } else {
            Err(ParserError::InvalidChar(input))
        }
    }
}

pub struct Line {
    pub line: String,
}

impl Line {
    pub fn parse(&self) -> Result<Parser, ParserError> {
        let mut parser = Parser::default();
        for c in self.line.chars() {
            match parser.feed(c) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(parser)
    }

    pub fn score_incomplete(&self) -> u64 {
        let result = self.parse();
        if result.is_err() {
            return 0;
        }
        let result = result.unwrap();
        if result.stack.is_empty() {
            return 0;
        }

        let mut score = 0u64;
        for c in result.stack.iter().rev() {
            score *= 5;
            score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("invalid char: {}", c),
            }
        }

        score
    }

    pub fn score_corrupt(&self) -> u64 {
        match self.parse() {
            Ok(_) => 0,
            Err(e) => match e {
                ParserError::InvalidClosingChar(c) => match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!("invalid InvalidClosingChar: {}", c),
                },
                ParserError::InvalidChar(_) => 0,
            },
        }
    }
}

pub struct Lines {
    pub lines: Vec<Line>,
}

impl std::str::FromStr for Lines {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .filter(|line| !line.trim_start().trim_end().is_empty())
            .map(str::to_string)
            .map(|line| Line { line })
            .collect();
        Ok(Lines { lines })
    }
}

impl Lines {
    pub fn total_score_corrupt(&self) -> u64 {
        self.lines.iter().map(|line| line.score_corrupt()).sum()
    }

    pub fn score_middle_incomplete(&self) -> u64 {
        let mut scores: Vec<u64> = self.lines.iter().map(|line| line.score_incomplete()).filter(|&score| score != 0).collect();
        if scores.len() % 2 != 1 {
            panic!("invalid number of incomplete scores");
        }
        scores.sort_unstable();
        scores[(scores.len() - 1) / 2]
    }
}

#[test]
fn test_parser() -> Result<(), error::Error> {
    let mut p = Parser::default();
    assert_eq!(p.feed('a'), Err(ParserError::InvalidChar('a')));
    assert_eq!(p.feed('('), Ok(()));
    assert_eq!(p.feed('{'), Ok(()));
    assert_eq!(p.feed('}'), Ok(()));
    assert_eq!(p.feed(')'), Ok(()));
    assert_eq!(p.feed('('), Ok(()));
    assert_eq!(p.feed('{'), Ok(()));
    assert_eq!(p.stack.len(), 2);
    assert_eq!(p.feed(']'), Err(ParserError::InvalidClosingChar(']')));
    assert_eq!(p.feed('}'), Ok(()));
    assert_eq!(p.feed(')'), Ok(()));
    assert_eq!(p.stack.len(), 0);
    assert_eq!(p.feed(')'), Err(ParserError::InvalidClosingChar(')')));
    Ok(())
}

#[test]
fn test_day10() -> Result<(), error::Error> {
    let input = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;
    let lines: Lines = input.parse()?;
    assert_eq!(lines.lines.len(), 10);
    assert_eq!(lines.lines[2].score_corrupt(), 1197);
    assert_eq!(lines.lines[4].score_corrupt(), 3);
    assert_eq!(lines.lines[5].score_corrupt(), 57);
    assert_eq!(lines.lines[7].score_corrupt(), 3);
    assert_eq!(lines.lines[8].score_corrupt(), 25137);
    assert_eq!(lines.total_score_corrupt(), 26397);

    assert_eq!(lines.lines[0].score_incomplete(), 288957);
    assert_eq!(lines.lines[1].score_incomplete(), 5566);
    assert_eq!(lines.lines[2].score_incomplete(), 0);
    assert_eq!(lines.lines[3].score_incomplete(), 1480781);
    assert_eq!(lines.lines[6].score_incomplete(), 995444);
    assert_eq!(lines.lines[9].score_incomplete(), 294);
    assert_eq!(lines.score_middle_incomplete(), 288957);

    let input = std::fs::read_to_string("input_day10")?;
    let lines: Lines = input.parse()?;
    assert_eq!(lines.lines.len(), 102);
    assert_eq!(lines.total_score_corrupt(), 288291);
    assert_eq!(lines.score_middle_incomplete(), 820045242);

    Ok(())
}
