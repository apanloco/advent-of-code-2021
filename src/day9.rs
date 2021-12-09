use crate::error;
use itertools::Itertools;

pub struct HeightMap {
    heightmap: Vec<Vec<i8>>,
}

impl std::str::FromStr for HeightMap {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let heightmap: Vec<Vec<i8>> = s
            .lines()
            .filter(|line| !line.trim_start().trim_end().is_empty())
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
            .collect();

        if heightmap.is_empty() || !heightmap.iter().all(|row| row.len() == heightmap[0].len()) {
            return Err(error::Error::Parse("invalid heightmap".to_string()));
        }

        Ok(HeightMap { heightmap })
    }
}

impl HeightMap {
    pub fn width(&self) -> i8 {
        self.heightmap[0].len() as i8
    }

    pub fn height(&self) -> i8 {
        self.heightmap.len() as i8
    }

    pub fn low_points(&self) -> Vec<(i8, i8)> {
        let mut low_points: Vec<(i8, i8)> = Vec::new();

        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.is_low_point(x, y) {
                    low_points.push((x, y));
                }
            }
        }

        low_points
    }

    pub fn sum_risk_levels(&self) -> u64 {
        self.low_points().iter().map(|(x, y)| self.at(*x, *y) as u64 + 1).sum()
    }

    pub fn basins(&self) -> Vec<i64> {
        self.low_points().into_iter().map(|(x, y)| self.basin_from_point(x, y)).collect()
    }

    pub fn largest_basins(&self) -> Vec<i64> {
        self.basins().into_iter().sorted_by(|a, b| b.cmp(a)).take(3).collect()
    }

    fn flow(&self, x: i8, y: i8, last_height: i8) -> Vec<(i8, i8)> {
        if self.is_oob(x, y) {
            return vec![];
        }

        let cur = self.at(x, y);

        if cur >= 9 || cur <= last_height {
            return vec![];
        }

        let mut points = vec![(x, y)];
        points.append(&mut self.flow(x, y - 1, cur));
        points.append(&mut self.flow(x + 1, y, cur));
        points.append(&mut self.flow(x, y + 1, cur));
        points.append(&mut self.flow(x - 1, y, cur));

        points
    }

    fn basin_from_point(&self, x: i8, y: i8) -> i64 {
        self.flow(x, y, -1).into_iter().unique().count() as i64
    }

    fn is_low_point(&self, x: i8, y: i8) -> bool {
        let current = self.at(x, y);

        self.is_point_higher_than_or_oob(x, y - 1, current)
            && self.is_point_higher_than_or_oob(x + 1, y, current)
            && self.is_point_higher_than_or_oob(x, y + 1, current)
            && self.is_point_higher_than_or_oob(x - 1, y, current)
    }

    fn is_oob(&self, x: i8, y: i8) -> bool {
        x < 0 || x >= self.width() || y < 0 || y >= self.height()
    }

    fn is_point_higher_than_or_oob(&self, x: i8, y: i8, value: i8) -> bool {
        self.is_oob(x, y) || self.at(x, y) > value
    }

    pub fn at(&self, x: i8, y: i8) -> i8 {
        self.heightmap[y as usize][x as usize]
    }
}

#[test]
fn test_day9() -> Result<(), error::Error> {
    let input = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    let heightmap: HeightMap = input.parse()?;
    assert_eq!(heightmap.width(), 10);
    assert_eq!(heightmap.height(), 5);
    assert_eq!(heightmap.at(0, 0), 2);
    assert_eq!(heightmap.at(9, 4), 8);
    assert_eq!(heightmap.low_points(), vec![(1, 0), (9, 0), (2, 2), (6, 4)]);
    assert_eq!(heightmap.sum_risk_levels(), 15);
    assert_eq!(heightmap.basin_from_point(1, 0), 3);
    assert_eq!(heightmap.basin_from_point(9, 0), 9);
    assert_eq!(heightmap.basin_from_point(2, 2), 14);
    assert_eq!(heightmap.basin_from_point(6, 4), 9);
    assert_eq!(heightmap.basins(), vec![3, 9, 14, 9]);
    assert_eq!(heightmap.largest_basins().iter().product::<i64>(), 1134);

    let input = std::fs::read_to_string("input_day9")?;
    let heightmap: HeightMap = input.parse()?;
    assert_eq!(heightmap.width(), 100);
    assert_eq!(heightmap.height(), 100);
    assert_eq!(heightmap.at(0, 0), 0);
    assert_eq!(heightmap.at(99, 99), 3);
    assert_eq!(heightmap.sum_risk_levels(), 526);
    assert_eq!(heightmap.largest_basins().iter().product::<i64>(), 1123524);

    Ok(())
}
