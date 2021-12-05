use crate::error;

#[derive(Clone)]
pub struct Number {
    pub number: u64,
    pub selected: bool,
}

#[derive(Clone)]
pub struct Board {
    pub matrix: Vec<Number>,
}

impl Board {
    pub fn from_numbers(numbers: Vec<u64>) -> Self {
        Board {
            matrix: numbers.iter().map(|n| Number { number: *n, selected: false }).collect(),
        }
    }

    fn mark(&mut self, number_to_mark: u64) {
        for number in &mut self.matrix {
            if number.number == number_to_mark {
                number.selected = true;
            }
        }
    }

    fn at(&self, x: u64, y: u64) -> &Number {
        let index = ((y * 5) + x) as usize;
        &self.matrix[index]
    }

    fn is_bingo_at_row(&self, r: u64) -> bool {
        let y = r;
        for x in 0..=4u64 {
            if !self.at(x, y).selected {
                return false;
            }
        }

        true
    }

    fn is_bingo_at_column(&self, c: u64) -> bool {
        let x = c;
        for y in 0..=4u64 {
            if !self.at(x, y).selected {
                return false;
            }
        }

        true
    }

    fn is_bingo(&self) -> bool {
        for x in 0..=4u64 {
            if self.is_bingo_at_column(x) {
                return true;
            }
        }

        for y in 0..=4u64 {
            if self.is_bingo_at_row(y) {
                return true;
            }
        }

        false
    }

    pub fn sum_unmarked(&self) -> u64 {
        self.matrix.iter().filter(|n| !n.selected).map(|n| n.number).sum()
    }

    fn _dump(&self) {
        for y in 0..=4u64 {
            for x in 0..=4u64 {
                let n = self.at(x, y);
                print!("{:4 }{}", n.number, if n.selected { "X" } else { "-" });
            }
            println!();
        }
        println!();
    }
}

pub struct Bingo {
    pub drawn_numbers: Vec<u64>,
    pub boards: Vec<Board>,
}

pub struct Winner {
    pub board: Board,
    pub winning_number: u64,
}

impl Winner {
    pub fn score(&self) -> u64 {
        self.winning_number * self.board.sum_unmarked()
    }
}

pub struct BingoResult {
    pub winners: Vec<Winner>,
}

pub fn play_bingo(mut bingo: Bingo) -> BingoResult {
    let mut winners: Vec<Winner> = Vec::with_capacity(bingo.boards.len());
    for drawn_number in bingo.drawn_numbers {
        for board in &mut bingo.boards {
            if !board.is_bingo() {
                board.mark(drawn_number);

                if board.is_bingo() {
                    winners.push(Winner {
                        board: board.clone(),
                        winning_number: drawn_number,
                    });
                }
            }
        }
    }
    BingoResult { winners }
}

fn parse_drawn_numbers(line: &str) -> Result<Vec<u64>, error::Error> {
    let result: Result<Vec<u64>, _> = line.split(',').map(|token| token.parse()).collect();
    Ok(result?)
}

pub fn parse_bingo(input: &str) -> Result<Bingo, error::Error> {
    let mut line_iterator = input.lines().filter(|l| !l.trim_start().trim_end().is_empty());
    let mut bingo = Bingo {
        drawn_numbers: parse_drawn_numbers(line_iterator.next().unwrap())?,
        boards: vec![],
    };
    for board_lines in line_iterator.collect::<Vec<&str>>().chunks(5) {
        let mut matrix: Vec<u64> = Vec::with_capacity(5 * 5);
        for board_line in board_lines {
            let numbers: Result<Vec<u64>, _> = board_line.split(' ').filter(|token| !token.trim_start().trim_end().is_empty()).map(|token| token.parse()).collect();
            matrix.append(&mut numbers?);
        }
        let board = Board::from_numbers(matrix);
        bingo.boards.push(board);
    }
    Ok(bingo)
}

#[test]
fn test_bingo() -> Result<(), error::Error> {
    let input = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
    "#;

    let bingo = parse_bingo(input)?;
    assert_eq!(bingo.drawn_numbers.len(), 27);
    assert_eq!(bingo.boards.len(), 3);
    assert_eq!(bingo.boards[0].matrix.len(), 5 * 5);

    assert_eq!(bingo.boards[0].at(0, 0).number, 22);
    assert_eq!(bingo.boards[0].at(4, 4).number, 19);

    assert_eq!(bingo.boards[1].at(0, 0).number, 3);
    assert_eq!(bingo.boards[1].at(4, 4).number, 6);

    assert_eq!(bingo.boards[2].at(0, 0).number, 14);
    assert_eq!(bingo.boards[2].at(4, 4).number, 7);

    let res = play_bingo(parse_bingo(input)?);
    assert_eq!(res.winners.len(), 3);
    assert_eq!(res.winners.len(), bingo.boards.len());

    let first_winner = &res.winners.first().unwrap();
    assert_eq!(first_winner.winning_number, 24);
    assert_eq!(first_winner.board.sum_unmarked(), 188);
    assert_eq!(first_winner.score(), 4512);

    Ok(())
}

#[test]
fn test_bingo_file() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("input_day4")?;

    let bingo = parse_bingo(&input)?;

    let res = play_bingo(parse_bingo(&input)?);
    assert!(!res.winners.is_empty());
    assert_eq!(res.winners.len(), bingo.boards.len());

    let first_winner = res.winners.first().unwrap();

    assert_eq!(first_winner.winning_number, 12);
    assert_eq!(first_winner.board.sum_unmarked(), 678);
    assert_eq!(first_winner.score(), 8136);

    let last_winner = res.winners.last().unwrap();
    assert_eq!(last_winner.winning_number, 66);
    assert_eq!(last_winner.board.sum_unmarked(), 193);
    assert_eq!(last_winner.score(), 12738);

    Ok(())
}
