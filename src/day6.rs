use crate::error;

#[derive(Debug, Clone)]
pub struct Fish {
    age: u64,
}

impl Fish {
    fn from_age(age: u64) -> Self {
        Fish { age }
    }
}

#[derive(Debug)]
pub struct FishGame {
    pub fish: Vec<Fish>,
}

impl std::str::FromStr for FishGame {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fish: Vec<Fish> = s
            .split(&[',', '\n'][..])
            .filter(|token| !token.trim_start().trim_end().is_empty())
            .map(|value_str| value_str.parse().unwrap())
            .map(Fish::from_age)
            .collect();
        Ok(FishGame { fish })
    }
}

impl FishGame {
    pub fn simulate_days(&self, days: u64) -> u64 {
        let mut fish = self.fish.clone();
        for _day in 0..days {
            let mut new_fish: Vec<Fish> = Vec::new();
            for mut fish in fish.iter_mut() {
                if fish.age == 0 {
                    fish.age = 7;
                    new_fish.push(Fish::from_age(8));
                }
                fish.age -= 1;
            }
            fish.append(&mut new_fish);
        }
        fish.len() as u64
    }

    pub fn simulate_days2(&self, days: u64) -> u64 {
        let mut buckets = vec![0u64; 9];
        for f in &self.fish {
            buckets[f.age as usize] += 1;
        }
        for _day in 0..days {
            let zeroes = buckets[0];

            // all fish one age less
            buckets[0] = buckets[1];
            buckets[1] = buckets[2];
            buckets[2] = buckets[3];
            buckets[3] = buckets[4];
            buckets[4] = buckets[5];
            buckets[5] = buckets[6];
            buckets[6] = buckets[7];
            buckets[7] = buckets[8];
            buckets[8] = 0;

            // create new fish
            buckets[8] += zeroes;
            buckets[6] += zeroes;
        }

        buckets.iter().sum()
    }
}

#[test]
fn test_fish_game() -> Result<(), error::Error> {
    let input = r#"
3,4,3,1,2"#;
    let game: FishGame = input.parse()?;
    assert_eq!(game.simulate_days(18), 26);
    assert_eq!(game.simulate_days2(18), 26);
    assert_eq!(game.simulate_days(80), 5934);
    assert_eq!(game.simulate_days2(80), 5934);
    assert_eq!(game.simulate_days2(256), 26984457539);

    let input = std::fs::read_to_string("input_day6")?;
    let game: FishGame = input.parse()?;
    assert_eq!(game.simulate_days(80), 396210);
    assert_eq!(game.simulate_days2(80), 396210);
    assert_eq!(game.simulate_days2(256), 1770823541496);

    Ok(())
}
