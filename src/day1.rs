use crate::error;

pub fn load_input(path: &str) -> Result<Vec<u64>, error::Error> {
    let data = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = data.lines().collect();
    let mut values = Vec::with_capacity(lines.len());
    for line in lines {
        values.push(line.parse()?);
    }
    Ok(values)
}

pub fn num_increased_measurements(input: &Vec<u64>) -> u64 {
    let mut last: Option<u64> = None;
    let mut num_increased = 0;
    for value in input {
        if let Some(last) = last {
            if *value > last {
                num_increased += 1;
            }
        }
        last = Some(*value);
    }
    num_increased
}

pub fn num_increased_measurements_window(input: &Vec<u64>) -> u64 {
    let mut last: Option<u64> = None;
    let mut num_increased = 0;

    for window in input.windows(3) {
        let value: u64 = window.iter().sum();
        if let Some(last) = last {
            if value > last {
                num_increased += 1;
            }
        }
        last = Some(value);
    }

    num_increased
}

#[test]
fn test_load_file() -> Result<(), error::Error> {
    let input: Vec<u64> = load_input("input_day1")?;
    assert_eq!(input.len(), 2000);
    Ok(())
}

#[test]
fn test_num_increased_measurements() {
    let input: Vec<u64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(num_increased_measurements(&input), 7);
}

#[test]
fn test_num_increased_measurements_window() {
    let input: Vec<u64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(num_increased_measurements_window(&input), 5);
}

#[test]
fn test_num_increased_measurements_file() -> Result<(), error::Error> {
    let input: Vec<u64> = load_input("input_day1")?;
    assert_eq!(num_increased_measurements(&input), 1759);
    Ok(())
}

#[test]
fn test_num_increased_measurements_window_file() -> Result<(), error::Error> {
    let input: Vec<u64> = load_input("input_day1")?;
    assert_eq!(num_increased_measurements_window(&input), 1805);
    Ok(())
}
