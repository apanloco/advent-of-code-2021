use crate::error;
use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

#[derive(PartialEq, Debug)]
pub struct Line {
    pub x1: u64,
    pub y1: u64,
    pub x2: u64,
    pub y2: u64,
}

impl Line {
    pub fn is_horizontal_or_vertical(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    pub fn points(&self) -> Vec<Point> {
        let delta_x: i64 = match self.x1.cmp(&self.x2) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let delta_y = match self.y1.cmp(&self.y2) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let mut points: Vec<Point> = Vec::new();

        let mut x: i64 = self.x1 as i64;
        let mut y: i64 = self.y1 as i64;

        loop {
            if x < 0 || y < 0 {
                panic!("negative x or y in point!");
            }

            points.push(Point { x: x as u64, y: y as u64 });

            if x == self.x2 as i64 && y == self.y2 as i64 {
                break;
            }

            x += delta_x;
            y += delta_y;
        }

        points
    }
}

impl std::str::FromStr for Line {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(&[',', ' ', '-', '>'][..]).filter(|line| !line.trim_start().trim_end().is_empty()).collect();
        if tokens.len() != 4 {
            Err(error::Error::Parse(format!("invalid line: {} tokens: {:?}", s, tokens)))
        } else {
            Ok(Line {
                x1: tokens[0].parse()?,
                y1: tokens[1].parse()?,
                x2: tokens[2].parse()?,
                y2: tokens[3].parse()?,
            })
        }
    }
}

pub fn load_lines_from_str(input: &str) -> Result<Vec<Line>, error::Error> {
    let mut lines: Vec<Line> = Vec::new();
    for line in input.lines() {
        if line.trim_start().trim_end().is_empty() {
            continue;
        }
        lines.push(line.parse()?);
    }
    Ok(lines)
}

pub struct LineMap {
    pub width: u64,
    pub height: u64,
    pub points: Vec<u64>,
}

impl LineMap {
    pub fn from_lines(lines: Vec<Line>) -> Self {
        let width = std::cmp::max(
            lines.iter().map(|line| line.x1).max_by(|lhs, rhs| lhs.cmp(rhs)),
            lines.iter().map(|line| line.x2).max_by(|lhs, rhs| lhs.cmp(rhs)),
        )
        .unwrap()
            + 1;
        let height = std::cmp::max(
            lines.iter().map(|line| line.y1).max_by(|lhs, rhs| lhs.cmp(rhs)),
            lines.iter().map(|line| line.y2).max_by(|lhs, rhs| lhs.cmp(rhs)),
        )
        .unwrap()
            + 1;

        if width == 0 || height == 0 {
            panic!("invalid size for LineMap");
        }

        let map_size = (width * height) as usize;

        let mut map = LineMap {
            width,
            height,
            points: vec![0; map_size],
        };

        for line in &lines {
            map.mark_line(line);
        }

        map
    }

    pub fn at(&self, x: u64, y: u64) -> u64 {
        let pos = (y * self.width + x) as usize;
        self.points[pos]
    }

    fn mark_point(&mut self, x: u64, y: u64) {
        let pos = (y * self.width + x) as usize;
        self.points[pos] += 1;
    }

    fn mark_line(&mut self, line: &Line) {
        let points: Vec<Point> = line.points();
        for point in points {
            self.mark_point(point.x, point.y);
        }
    }

    pub fn num_points_overlap(&self) -> u64 {
        self.points.iter().filter(|&p| p > &1u64).count() as u64
    }
}

#[test]
fn test_load_lines() -> Result<(), error::Error> {
    let input = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
    let lines = load_lines_from_str(input)?;
    assert_eq!(lines.len(), 10);
    assert_eq!(lines.first().unwrap().x1, 0);
    assert_eq!(lines.first().unwrap().y1, 9);
    assert_eq!(lines.first().unwrap().x2, 5);
    assert_eq!(lines.first().unwrap().y2, 9);
    assert_eq!(lines.last().unwrap().x1, 5);
    assert_eq!(lines.last().unwrap().y1, 5);
    assert_eq!(lines.last().unwrap().x2, 8);
    assert_eq!(lines.last().unwrap().y2, 2);

    assert_eq!(load_lines_from_str(" 123  ,  456 -   >  911,119")?, vec![Line { x1: 123, y1: 456, x2: 911, y2: 119 }]);

    Ok(())
}

#[test]
fn test_line() -> Result<(), error::Error> {
    assert!(!Line { x1: 123, y1: 456, x2: 911, y2: 119 }.is_horizontal_or_vertical());

    assert!(Line { x1: 123, y1: 456, x2: 123, y2: 119 }.is_horizontal_or_vertical());

    assert_eq!(Line { x1: 123, y1: 456, x2: 911, y2: 456 }.is_horizontal_or_vertical(), true);

    assert_eq!(Line { x1: 1, y1: 2, x2: 3, y2: 4 }.points(), vec![Point { x: 1, y: 2 }, Point { x: 2, y: 3 }, Point { x: 3, y: 4 }]);

    Ok(())
}

#[test]
fn test_complete() -> Result<(), error::Error> {
    let input = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
    let lines = load_lines_from_str(input)?;
    let lines = lines.into_iter().filter(|line| line.is_horizontal_or_vertical()).collect();
    let map = LineMap::from_lines(lines);

    assert_eq!(map.width, 10);
    assert_eq!(map.height, 10);

    assert_eq!(map.at(7, 0), 1);
    assert_eq!(map.at(0, 9), 2);

    #[rustfmt::skip]
    assert_eq!(
        map.points,
        vec![
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 1, 1, 2, 1, 1, 1, 2, 1, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            2, 2, 2, 1, 1, 1, 0, 0, 0, 0,
        ],
    );

    assert_eq!(map.num_points_overlap(), 5);

    let lines = load_lines_from_str(input)?;
    let map = LineMap::from_lines(lines);

    #[rustfmt::skip]
    assert_eq!(
        map.points,
        vec![
            1, 0, 1, 0, 0, 0, 0, 1, 1, 0,
            0, 1, 1, 1, 0, 0, 0, 2, 0, 0,
            0, 0, 2, 0, 1, 0, 1, 1, 1, 0,
            0, 0, 0, 1, 0, 2, 0, 2, 0, 0,
            0, 1, 1, 2, 3, 1, 3, 2, 1, 1,
            0, 0, 0, 1, 0, 2, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 0, 1, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 1, 0,
            2, 2, 2, 1, 1, 1, 0, 0, 0, 0,
        ],
    );

    assert_eq!(map.num_points_overlap(), 12);

    Ok(())
}

#[test]
fn test_day5() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("input_day5")?;
    let lines = load_lines_from_str(&input)?;
    let lines = lines.into_iter().filter(|line| line.is_horizontal_or_vertical()).collect();
    let map = LineMap::from_lines(lines);

    assert_eq!(map.width, 988);
    assert_eq!(map.height, 990);

    assert_eq!(map.num_points_overlap(), 5306);

    let lines = load_lines_from_str(&input)?;
    let map = LineMap::from_lines(lines);

    assert_eq!(map.num_points_overlap(), 17787);

    Ok(())
}
