use std::fmt::{Display, Formatter};
use crate::error;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Range3D {
    x_begin: i64,
    x_end: i64,
    y_begin: i64,
    y_end: i64,
    z_begin: i64,
    z_end: i64,
}

impl Range3D {
    pub fn is_superset_of(&self, rhs: &Range3D) -> bool {
        self.x_begin <= rhs.x_begin && self.x_end >= rhs.x_end &&
            self.y_begin <= rhs.y_begin && self.y_end >= rhs.y_end &&
            self.z_begin <= rhs.z_begin && self.z_end >= rhs.z_end
    }

    pub fn size(&self) -> usize {
        (self.x_end - self.x_begin + 1) as usize * (self.y_end - self.y_begin + 1) as usize * (self.z_end - self.z_begin + 1) as usize
    }

    fn cut(&self, against: &Vec<Range3D>) -> Vec<Range3D> {
        let mut x_cut: Vec<i64> = vec![self.x_begin, self.x_end + 1];
        let mut y_cut: Vec<i64> = vec![self.y_begin, self.y_end + 1];
        let mut z_cut: Vec<i64> = vec![self.z_begin, self.z_end + 1];

        for range in against.iter() {
            x_cut.push(range.x_begin);
            x_cut.push(range.x_end + 1);
            y_cut.push(range.y_begin);
            y_cut.push(range.y_end + 1);
            z_cut.push(range.z_begin);
            z_cut.push(range.z_end + 1);
        }

        x_cut = x_cut.into_iter().filter(|&x| x >= self.x_begin && x <= self.x_end + 1).collect();
        y_cut = y_cut.into_iter().filter(|&y| y >= self.y_begin && y <= self.y_end + 1).collect();
        z_cut = z_cut.into_iter().filter(|&z| z >= self.z_begin && z <= self.z_end + 1).collect();

        x_cut.sort();
        x_cut.dedup();
        y_cut.sort();
        y_cut.dedup();
        z_cut.sort();
        z_cut.dedup();

        println!("x_cut: {:?}", x_cut);
        println!("y_cut: {:?}", y_cut);
        println!("z_cut: {:?}", z_cut);

        let mut ranges = vec![];

        for x in x_cut.windows(2) {
            for y in y_cut.windows(2) {
                for z in z_cut.windows(2) {
                    ranges.push(Range3D {
                        x_begin: x[0],
                        x_end: x[1] - 1,
                        y_begin: y[0],
                        y_end: y[1] - 1,
                        z_begin: z[0],
                        z_end: z[1] - 1,
                    })
                }
            }
        }

        println!("ranges: {:?}", ranges);

        ranges
    }
}

#[derive(Debug)]
pub struct Grid {
    ranges: Vec<Range3D>,
}

impl Grid {
    pub fn num_lit(&self) -> usize {
        self.ranges.iter().map(|r| r.size()).sum()
    }
}

impl Grid {
    fn already_lit(&self, new: &Range3D) -> bool {
        !self.ranges.is_empty() && self.ranges.iter().all(|r| r.is_superset_of(new))
    }

    fn add_range(&mut self, new: &Range3D) {
        let cuts = new.cut(&self.ranges);
        for cut in cuts.iter() {
            if self.already_lit(cut) {
                continue;
            }

            self.ranges.push(*cut);
        }
    }

    fn remove_range(&mut self, _range: &Range3D) {}
}

impl std::str::FromStr for Range3D {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_ignore, x_begin, x_end, y_begin, y_end, z_begin, z_end) =
            scan_fmt::scan_fmt!(s, "{} x={d}..{d},y={d}..{d},z={d}..{d}", String, i64, i64, i64, i64, i64, i64)?;
        Ok(Self {
            x_begin: i64::min(x_begin, x_end),
            x_end: i64::max(x_begin, x_end),
            y_begin: i64::min(y_begin, y_end),
            y_end: i64::max(y_begin, y_end),
            z_begin: i64::min(z_begin, z_end),
            z_end: i64::max(z_begin, z_end),
        })
    }
}

impl std::str::FromStr for Grid {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid {
            ranges: vec![]
        };

        for line in s.lines().map(|l| l.trim_start().trim_end()).filter(|l| !l.is_empty()) {
            if line.starts_with("on") {
                println!("adding: {}", line);
                grid.add_range(&line.parse()?);
            } else if line.starts_with("off") {
                println!("removing: {}", line);
                grid.remove_range(&line.parse()?);
            } else {
                if !line.starts_with("#") {
                    panic!("invalid line: {}", line);
                }
                println!("ignoring: {}", line);
            }
        }

        Ok(grid)
    }
}

impl Display for Range3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{} {}:{} {}:{} => {}]",
               self.x_begin, self.x_end, self.y_begin, self.y_end, self.z_begin, self.z_end, self.size())
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GRID {} {}", self.ranges.len(), self.num_lit())?;
        for range in self.ranges.iter() {
            writeln!(f, "  {}", range)?;
        }
        Ok(())
    }
}

#[test]
fn test_range() -> Result<(), error::Error> {
    let r: Range3D = "on x=10..12,y=10..12,z=10..12".parse()?;
    assert_eq!(r.size(), 3 * 3 * 3);
    let r: Range3D = "off x=-32..-23,y=11..30,z=-14..3".parse()?;
    assert_eq!(r.size(), 10 * 20 * 18);
    let r: Range3D = "on x=1..1,y=1..1,z=1..1".parse()?;
    assert_eq!(r.size(), 1);

    let r: Range3D = "on x=0..1,y=0..1,z=0..1".parse()?;
    let r2: Range3D = "on x=1..1,y=1..1,z=1..1".parse()?;
    let r3: Range3D = "on x=1..1,y=1..1,z=1..3".parse()?;
    let r4: Range3D = "on x=-1..1,y=0..1,z=0..1".parse()?;

    assert!(r.is_superset_of(&r2));
    assert!(!r.is_superset_of(&r3));
    assert!(!r.is_superset_of(&r4));

    Ok(())
}

#[test]
fn test_range_cut() -> Result<(), error::Error> {
    let r1: Range3D = "on x=0..1,y=0..1,z=0..1".parse()?;
    let r2: Range3D = "on x=1..1,y=1..1,z=1..1".parse()?;
    let cut = r1.cut(&vec![r2]);
    assert_eq!(cut.len(), 8);
    let cut = r2.cut(&vec![r1]);
    assert_eq!(cut.len(), 1);

    let r1: Range3D = "on x=0..0,y=0..0,z=0..0".parse()?;
    let r2: Range3D = "on x=1..1,y=1..1,z=1..1".parse()?;
    let cut = r1.cut(&vec![r2]);
    assert_eq!(cut.len(), 1);

    Ok(())
}

#[test]
fn test_grid() -> Result<(), error::Error> {
    let g = Grid {
        ranges: vec![]
    };

    let r1: Range3D = "on x=0..1,y=0..1,z=0..1".parse()?;
    let r2: Range3D = "on x=1..1,y=1..1,z=1..1".parse()?;
    let r3: Range3D = "on x=1..1,y=1..1,z=1..2".parse()?;
    let r4: Range3D = "on x=1..1,y=-1..1,z=1..1".parse()?;
    let r5: Range3D = "on x=-1..1,y=1..1,z=1..1".parse()?;

    assert!(!g.already_lit(&r1));

    let g = Grid {
        ranges: vec![r1]
    };

    assert!(g.already_lit(&r1));
    assert!(g.already_lit(&r2));
    assert!(!g.already_lit(&r3));
    assert!(!g.already_lit(&r4));
    assert!(!g.already_lit(&r5));

    Ok(())
}

#[test]
fn test_day22() -> Result<(), error::Error> {
    let input = r#"
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
#off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"#;
    let grid: Grid = input.parse()?;
    println!("{}", &grid);
    assert_eq!(grid.num_lit(), 39);
    Ok(())
}
