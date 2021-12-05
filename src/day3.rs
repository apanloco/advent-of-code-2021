pub fn count_01(nums: &Vec<String>, index: usize) -> (u64, u64) {
    let mut count_0s: u64 = 0;
    let mut count_1s: u64 = 0;

    for number in nums {
        match number.chars().nth(index).unwrap() {
            '0' => count_0s += 1,
            '1' => count_1s += 1,
            _ => {
                panic!("bug");
            }
        }
    }

    (count_0s, count_1s)
}

pub struct PowerConsumption {
    gamma_rate: u64,
    epsilon_rate: u64,
}

impl PowerConsumption {
    pub fn sum(&self) -> u64 {
        self.gamma_rate * self.epsilon_rate
    }
}

pub struct LifeSupport {
    oxygen: u64,
    co2: u64,
}

impl LifeSupport {
    pub fn sum(&self) -> u64 {
        self.oxygen * self.co2
    }
}

pub fn calculate_power_consumption(numbers: &Vec<String>) -> PowerConsumption {
    if numbers.is_empty() {
        panic!("no numbers");
    }

    let mut pc = PowerConsumption { gamma_rate: 0, epsilon_rate: 0 };

    let mut gamma = String::new();
    let mut epsilon = String::new();

    let mut index = 0;
    while index < numbers[0].len() {
        let (count_0s, count_1s) = count_01(numbers, index);

        if count_0s == count_1s {
            panic!("bad algo");
        }

        if count_1s > count_0s {
            gamma.push('1');
            epsilon.push('0')
        } else {
            gamma.push('0');
            epsilon.push('1')
        }

        index += 1;
    }

    pc.gamma_rate = u64::from_str_radix(&gamma, 2).unwrap();
    pc.epsilon_rate = u64::from_str_radix(&epsilon, 2).unwrap();

    pc
}

pub fn calculate_life_support(numbers: &Vec<String>) -> LifeSupport {
    if numbers.is_empty() {
        panic!("no numbers");
    }

    let mut ls = LifeSupport { oxygen: 0, co2: 0 };

    let mut oxygen_nums = numbers.to_owned();
    let mut co2_nums = numbers.to_owned();

    let mut index = 0;
    while index < numbers[0].len() {
        if oxygen_nums.len() > 1 {
            let (count_0s_oxygen, count_1s_oxygen) = count_01(&oxygen_nums, index);

            let keep_oxygen = if count_1s_oxygen >= count_0s_oxygen { '1' } else { '0' };

            oxygen_nums.retain(|num| num.chars().nth(index).unwrap() == keep_oxygen);
        }

        if co2_nums.len() > 1 {
            let (count_0s_co2, count_1s_co2) = count_01(&co2_nums, index);

            let keep_co2 = if count_0s_co2 <= count_1s_co2 { '0' } else { '1' };

            co2_nums.retain(|num| num.chars().nth(index).unwrap() == keep_co2);
        }

        if oxygen_nums.len() == 1 && co2_nums.len() == 1 {
            break;
        }

        index += 1;
    }

    ls.oxygen = u64::from_str_radix(&oxygen_nums[0], 2).unwrap();
    ls.co2 = u64::from_str_radix(&co2_nums[0], 2).unwrap();

    ls
}

#[test]
fn test_power_consumption() {
    let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    let nums: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let res = calculate_power_consumption(&nums);

    assert_eq!(res.gamma_rate, 22);
    assert_eq!(res.epsilon_rate, 9);
    assert_eq!(res.sum(), 198);

    let input = std::fs::read_to_string("input_day3").unwrap();
    let nums: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let res = calculate_power_consumption(&nums);

    assert_eq!(res.gamma_rate, 2601);
    assert_eq!(res.epsilon_rate, 1494);
    assert_eq!(res.sum(), 3885894);
}

#[test]
fn test_life_support() {
    let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    let nums: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let res = calculate_life_support(&nums);

    assert_eq!(res.oxygen, 23);
    assert_eq!(res.co2, 10);
    assert_eq!(res.sum(), 230);

    let input = std::fs::read_to_string("input_day3").unwrap();
    let nums: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let res = calculate_life_support(&nums);

    assert_eq!(res.oxygen, 3775);
    assert_eq!(res.co2, 1159);
    assert_eq!(res.sum(), 4375225);
}
