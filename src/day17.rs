use crate::error;

use scan_fmt;

pub struct TargetArea {
    x_begin: i64,
    x_end: i64,
    y_begin: i64,
    y_end: i64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl std::str::FromStr for TargetArea {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start().trim_end();
        let (x_begin, x_end, y_begin, y_end) = scan_fmt::scan_fmt!(s, "target area: x={d}..{d}, y={d}..{d}", i64, i64, i64, i64)?;
        Ok(Self {
            x_begin: std::cmp::min(x_begin, x_end),
            x_end: std::cmp::max(x_begin, x_end),
            y_begin: std::cmp::min(y_begin, y_end),
            y_end: std::cmp::max(y_begin, y_end),
        })
    }
}

impl TargetArea {
    fn simulate_trajectory(&self, initial_position: &Pos, mut velocity_x: i64, mut velocity_y: i64) -> Vec<Pos> {
        let mut position = *initial_position;
        let mut positions: Vec<Pos> = vec![position];
        loop {
            position.x += velocity_x;
            position.y += velocity_y;

            positions.push(position);

            if velocity_x == 0 && (position.x < self.x_begin || position.x > self.x_end) {
                break;
            }

            if position.y < std::cmp::min(self.y_end, self.y_begin) {
                break;
            }

            // modify velocities

            velocity_x += match velocity_x.cmp(&0) {
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Greater => -1,
                _ => 0,
            };

            velocity_y -= 1;
        }

        positions
    }

    fn inside_target_area(&self, pos: &Pos) -> bool {
        self.x_begin <= pos.x && pos.x <= self.x_end && self.y_begin <= pos.y && pos.y <= self.y_end
    }

    fn inside_target_area_x(&self, pos: i64) -> bool {
        self.x_begin <= pos && pos <= self.x_end
    }

    fn inside_target_area_y(&self, pos: i64) -> bool {
        self.y_begin <= pos && pos <= self.y_end
    }

    pub fn hits_target(&self, trajectory: &Vec<Pos>) -> Option<Pos> {
        for pos in trajectory {
            let is_inside = self.inside_target_area(pos);
            if is_inside {
                return Some(*pos);
            }
        }
        None
    }

    fn would_hit_x(&self, initial_position: i64, mut velocity: i64) -> bool {
        let mut position = initial_position;
        loop {
            if self.inside_target_area_x(position) {
                return true;
            }

            position += velocity;

            if velocity > 0 {
                velocity -= 1;
            }

            if velocity < 0 {
                velocity += 1;
            }

            if velocity == 0 {
                break;
            }
        }
        false
    }

    fn would_hit_y(&self, initial_position: i64, mut velocity: i64) -> bool {
        let mut position = initial_position;
        loop {
            if self.inside_target_area_y(position) {
                return true;
            }

            position += velocity;

            velocity -= 1;

            if position < std::cmp::min(self.y_end, self.y_begin) {
                break;
            }
        }
        false
    }

    fn find_possible_velocities_x(&self, initial_position: i64) -> Vec<i64> {
        let max_velocity = std::cmp::max(self.x_end - initial_position, self.x_begin - initial_position);
        let min_velocity = 0;
        let mut possible_velocities = Vec::new();
        for possible_velocity in std::cmp::min(min_velocity, max_velocity)..=std::cmp::max(min_velocity, max_velocity) {
            if self.would_hit_x(initial_position, possible_velocity) {
                possible_velocities.push(possible_velocity);
            }
        }
        possible_velocities
    }

    fn find_possible_velocities_y(&self, initial_position: i64) -> Vec<i64> {
        let max_velocity = std::cmp::max((self.y_end - initial_position).abs(), (self.y_begin - initial_position).abs());
        let min_velocity = -std::cmp::max((self.y_end - initial_position).abs(), (self.y_begin - initial_position).abs());
        let mut possible_velocities = Vec::new();
        for possible_velocity in std::cmp::min(min_velocity, max_velocity)..=std::cmp::max(min_velocity, max_velocity) {
            if self.would_hit_y(initial_position, possible_velocity) {
                possible_velocities.push(possible_velocity);
            }
        }
        possible_velocities
    }

    pub fn all_initial_velocities(&self, initial_position: Pos) -> Vec<(i64, i64)> {
        let x_velocities = self.find_possible_velocities_x(initial_position.x);
        let y_velocities = self.find_possible_velocities_y(initial_position.y);
        let mut velocities: Vec<(i64, i64)> = Vec::new();
        for &y_vel in y_velocities.iter().rev() {
            for &x_vel in x_velocities.iter() {
                let trajectory = self.simulate_trajectory(&initial_position, x_vel, y_vel);
                if trajectory.iter().any(|pos| self.inside_target_area(pos)) {
                    velocities.push((x_vel, y_vel));
                }
            }
        }
        velocities.into_iter().collect()
    }

    pub fn optimum_trajectory(&self, initial_position: Pos) -> Option<Vec<Pos>> {
        let x_velocities = self.find_possible_velocities_x(initial_position.x);
        let y_velocities = self.find_possible_velocities_y(initial_position.y);
        for &y_vel in y_velocities.iter().rev() {
            for &x_vel in x_velocities.iter() {
                let trajectory = self.simulate_trajectory(&initial_position, x_vel, y_vel);
                if trajectory.iter().any(|pos| self.inside_target_area(pos)) {
                    return Some(trajectory);
                }
            }
        }
        None
    }
}

#[test]
fn test_find_possible_velocities() -> Result<(), error::Error> {
    let target_area: TargetArea = "target area: x=-5..-5, y=-5..-5".parse()?;
    assert!(target_area.inside_target_area(&Pos::new(-5, -5)));

    let velocities = target_area.find_possible_velocities_x(-1);
    assert_eq!(velocities, vec![-4]);

    let velocities = target_area.find_possible_velocities_y(-1);
    assert_eq!(velocities, vec![-4, 3]);

    let target_area: TargetArea = "target area: x=20..30, y=-10..-5".parse()?;
    let velocities = target_area.find_possible_velocities_y(0);
    assert!(velocities.iter().any(|&v| v == 9));

    Ok(())
}

#[test]
fn test_ranges() -> Result<(), error::Error> {
    let a = -5i64;
    let b = -10i64;
    let v: Vec<i64> = (b as i64..=a as i64).collect();
    assert_eq!(v.len(), 6);
    let v: Vec<i64> = (a as i64..=b as i64).collect();
    assert_eq!(v.len(), 0);
    Ok(())
}

#[test]
fn test_target_area() -> Result<(), error::Error> {
    let target_area: TargetArea = "target area: x=-20..-10, y=10..15".parse()?;
    assert!(target_area.inside_target_area(&Pos::new(-20, 10)));
    assert!(target_area.inside_target_area(&Pos::new(-10, 10)));
    assert!(target_area.inside_target_area(&Pos::new(-10, 15)));
    assert!(target_area.inside_target_area(&Pos::new(-20, 15)));
    assert!(target_area.inside_target_area(&Pos::new(-15, 12)));
    assert!(!target_area.inside_target_area(&Pos::new(-21, 10)));
    assert!(!target_area.inside_target_area(&Pos::new(-10, 9)));
    assert!(!target_area.inside_target_area(&Pos::new(-9, 15)));
    assert!(!target_area.inside_target_area(&Pos::new(-20, 16)));

    let target_area: TargetArea = "target area: x=20..30, y=-10..-5".parse()?;
    assert!(target_area.inside_target_area(&Pos::new(28, -7)));

    let trajectory = target_area.simulate_trajectory(&Pos::new(0, 0), 7, 2);
    assert_eq!(target_area.hits_target(&trajectory), Some(Pos::new(28, -7)));

    let trajectory = target_area.simulate_trajectory(&Pos::new(0, 0), 6, 3);
    assert_eq!(target_area.hits_target(&trajectory), Some(Pos::new(21, -9)));

    let trajectory = target_area.simulate_trajectory(&Pos::new(0, 0), 9, 0);
    assert_eq!(target_area.hits_target(&trajectory), Some(Pos::new(30, -6)));

    let trajectory = target_area.simulate_trajectory(&Pos::new(0, 0), 17, -4);
    assert_eq!(target_area.hits_target(&trajectory), None);

    Ok(())
}

#[test]
fn test_day17() -> Result<(), error::Error> {
    let target_area: TargetArea = "target area: x=20..30, y=-10..-5".parse()?;
    assert_eq!(target_area.x_begin, 20);
    assert_eq!(target_area.x_end, 30);
    assert_eq!(target_area.y_begin, -10);
    assert_eq!(target_area.y_end, -5);
    let trajectory: Vec<Pos> = target_area.optimum_trajectory(Pos::new(0, 0)).unwrap();
    assert_eq!(trajectory.iter().map(|p| p.y).max().unwrap(), 45);
    let all_initial_velocities = target_area.all_initial_velocities(Pos::new(0, 0));
    assert_eq!(all_initial_velocities.len(), 112);

    let target_area: TargetArea = std::fs::read_to_string("input_day17")?.parse()?;
    let trajectory: Vec<Pos> = target_area.optimum_trajectory(Pos::new(0, 0)).unwrap();
    assert_eq!(trajectory.iter().map(|p| p.y).max().unwrap(), 5151);
    let all_initial_velocities = target_area.all_initial_velocities(Pos::new(0, 0));
    assert_eq!(all_initial_velocities.len(), 968);

    Ok(())
}
