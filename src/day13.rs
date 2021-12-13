use crate::error;

use itertools::Itertools;

#[derive(Clone)]
pub enum FoldType {
    Vertical,
    Horizontal,
}

#[derive(Clone)]
pub struct FoldInstruction {
    fold_at_line: usize,
    fold_type: FoldType,
}

pub struct Paper {
    points: Vec<(usize, usize)>,
    instructions: Vec<FoldInstruction>,
}

impl Paper {
    fn width(&self) -> usize {
        self.points.iter().map(|&p| p.0).max().unwrap() as usize + 1
    }

    fn height(&self) -> usize {
        self.points.iter().map(|&p| p.1).max().unwrap() as usize + 1
    }

    pub fn fold_once(&self) -> Paper {
        let instruction = &self.instructions[0];
        Paper {
            points: self
                .points
                .iter()
                .map(|&p| match instruction.fold_type {
                    FoldType::Vertical => {
                        let x = if p.0 < instruction.fold_at_line {
                            p.0
                        } else {
                            (instruction.fold_at_line - 1) - (p.0 - instruction.fold_at_line - 1)
                        };
                        let y = p.1;
                        (x, y)
                    }
                    FoldType::Horizontal => {
                        let x = p.0;
                        let y = if p.1 < instruction.fold_at_line {
                            p.1
                        } else {
                            (instruction.fold_at_line - 1) - (p.1 - instruction.fold_at_line - 1)
                        };
                        (x, y)
                    }
                })
                .unique()
                .collect(),
            instructions: self.instructions[1..].to_vec(),
        }
    }

    fn plot(&self) -> Vec<Vec<u8>> {
        let mut map = vec![vec![0; self.width()]; self.height()];
        for (x, y) in &self.points {
            map[*y][*x] += 1;
        }
        map
    }

    pub fn dump(&self) {
        let map: Vec<Vec<u8>> = self.plot();
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                print!("{}", if map[y][x] == 0 { ' ' } else { 'X' });
            }
            println!();
        }
        println!();
        println!();
    }
}

impl std::str::FromStr for FoldInstruction {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // fold along x=655
        let s = &s[11..];
        let mut tokens = s.split('=');
        let x_or_y = tokens.next().unwrap();
        let line: usize = tokens.next().unwrap().parse()?;
        Ok(Self {
            fold_at_line: line,
            fold_type: if x_or_y == "x" { FoldType::Vertical } else { FoldType::Horizontal },
        })
    }
}

impl std::str::FromStr for Paper {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut paper = Paper { points: vec![], instructions: vec![] };

        for line in s.lines().filter(|l| !l.trim_start().trim_end().is_empty()) {
            if line.starts_with("fold along") {
                paper.instructions.push(line.parse()?);
            } else {
                // 1288,245
                let mut tokens = line.split(',');
                let x: usize = tokens.next().unwrap().parse()?;
                let y: usize = tokens.next().unwrap().parse()?;
                paper.points.push((x, y));
            }
        }

        Ok(paper)
    }
}

#[test]
fn test_day13() -> Result<(), error::Error> {
    let input = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    let paper: Paper = input.parse()?;
    let paper = paper.fold_once();
    assert_eq!(paper.points.iter().count(), 17);
    let paper = paper.fold_once();
    paper.dump();

    let paper: Paper = std::fs::read_to_string("input_day13")?.parse()?;
    let paper = paper.fold_once();
    assert_eq!(paper.points.iter().count(), 759);
    let paper = paper.fold_once();
    let paper = paper.fold_once();
    let paper = paper.fold_once();
    let paper = paper.fold_once();
    let paper = paper.fold_once();
    let paper = paper.fold_once();
    let paper = paper.fold_once();
    let paper = paper.fold_once();
    let paper = paper.fold_once();
    let paper = paper.fold_once();
    let paper = paper.fold_once();
    paper.dump();

    Ok(())
}
