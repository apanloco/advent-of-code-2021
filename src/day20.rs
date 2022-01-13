use crate::error;
use std::collections::HashMap;

pub struct Image {
    pub enhancement: String,
    pub pixels: HashMap<i64, Vec<i64>>,
    pub oob_index: usize,
}

#[derive(Debug)]
enum PixelEnhancementResult {
    Dark,
    Light,
}

impl Image {
    pub fn num_lit_pixels(&self) -> usize {
        self.pixels.iter().map(|(_, v)| v.len()).sum()
    }

    fn minmax_x(&self) -> (i64, i64) {
        let mut min = None;
        let mut max = None;
        for (_, v) in self.pixels.iter() {
            if let Some(&first) = v.first() {
                if min.is_none() || min.unwrap() > first {
                    min = Some(first);
                }
            }
            if let Some(&last) = v.last() {
                if max.is_none() || max.unwrap() < last {
                    max = Some(last);
                }
            }
        }
        (min.expect("no pixels"), max.expect("no pixels"))
    }

    fn minmax_y(&self) -> (i64, i64) {
        let mut min = None;
        let mut max = None;
        for (&k, _) in self.pixels.iter() {
            if min.is_none() || min.unwrap() > k {
                min = Some(k);
            }
            if max.is_none() || max.unwrap() < k {
                max = Some(k);
            }
        }
        (min.expect("no pixels"), max.expect("no pixels"))
    }

    pub fn is_lit(&self, x: i64, y: i64) -> bool {
        if let Some(vec) = self.pixels.get(&y) {
            vec.contains(&x)
        } else {
            false
        }
    }

    pub fn draw(&self) {
        let (x_start, x_end) = self.minmax_x();
        let (y_start, y_end) = self.minmax_y();
        for y in y_start..=y_end {
            for x in x_start..=x_end {
                print!("{}", if self.is_lit(x, y) { "#" } else { "." });
            }
            println!();
        }
        println!();
    }

    fn add_pixel(&mut self, x: i64, y: i64) {
        let vec = self.pixels.entry(y).or_default();
        vec.push(x);
        vec.sort_unstable();
    }

    fn next_oob_index(enhancement: &str, cur_index: usize) -> usize {
        if enhancement.as_bytes()[0] == b'#' {
            if cur_index == 0 {
                511
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn enhance(&self) -> Self {
        let (min_x, max_x) = self.minmax_x();
        let (min_y, max_y) = self.minmax_y();

        let mut image = Image {
            enhancement: self.enhancement.clone(),
            pixels: HashMap::new(),
            oob_index: Image::next_oob_index(&self.enhancement, self.oob_index),
        };

        for y in (min_y - 1)..=(max_y + 1) {
            for x in (min_x - 1)..=(max_x + 1) {
                match self.enhance_pixel(x, y, min_x, max_x, min_y, max_y) {
                    PixelEnhancementResult::Dark => {}
                    PixelEnhancementResult::Light => {
                        image.add_pixel(x, y);
                    }
                }
            }
        }

        image
    }

    fn enhance_pixel(&self, x: i64, y: i64, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> PixelEnhancementResult {
        let mut index_string = String::with_capacity(9);
        for y in (y - 1)..=(y + 1) {
            for x in (x - 1)..=(x + 1) {
                if x < min_x || x > max_x || y < min_y || y > max_y {
                    index_string += match self.enhancement.as_bytes()[self.oob_index] {
                        b'.' => "0",
                        b'#' => "1",
                        _ => panic!("invalid something"),
                    };
                } else {
                    index_string += if self.is_lit(x, y) { "1" } else { "0" };
                }
            }
        }

        let index = usize::from_str_radix(&index_string, 2).unwrap();

        let result = match self.enhancement.as_bytes()[index] {
            b'#' => PixelEnhancementResult::Light,
            b'.' => PixelEnhancementResult::Dark,
            _ => panic!("invalid enhancement"),
        };

        result
    }
}

impl std::str::FromStr for Image {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut image = Image {
            enhancement: String::new(),
            pixels: HashMap::new(),
            oob_index: 0,
        };

        enum State {
            Enhancement,
            Image,
        }

        let mut state = State::Enhancement;

        let mut line_index = 0;

        for line in s.lines().map(|l| l.trim_start().trim_end()) {
            if line.is_empty() && !image.enhancement.is_empty() {
                state = State::Image;
                continue;
            }
            match state {
                State::Enhancement => {
                    image.enhancement.push_str(line);
                }
                State::Image => {
                    for (index, char) in line.chars().enumerate() {
                        match char {
                            '#' => image.add_pixel(index as i64, line_index),
                            '.' => {}
                            _ => panic!("invalid input"),
                        }
                    }
                    line_index += 1;
                }
            }
        }

        image.oob_index = Image::next_oob_index(&image.enhancement, image.oob_index);

        Ok(image)
    }
}

#[test]
fn test_day19() -> Result<(), error::Error> {
    let input = r#"
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;

    let image: Image = input.parse()?;

    assert_eq!(image.enhancement.len(), 512);
    assert_eq!(image.pixels.len(), 5);
    assert_eq!(image.pixels.iter().map(|(_, v)| v.len()).sum::<usize>(), 10);
    assert_eq!(image.minmax_x(), (0, 4));
    assert_eq!(image.minmax_y(), (0, 4));
    assert_eq!(image.num_lit_pixels(), 10);
    let image = image.enhance();
    assert_eq!(image.num_lit_pixels(), 24);
    let image = image.enhance();
    assert_eq!(image.num_lit_pixels(), 35);

    let mut image: Image = std::fs::read_to_string("input_day20")?.parse()?;
    assert_eq!(image.enhancement.len(), 512);
    assert_eq!(image.minmax_x(), (0, 99));
    assert_eq!(image.minmax_y(), (0, 99));
    assert_eq!(image.num_lit_pixels(), 5023);
    image = image.enhance();
    image = image.enhance();
    assert_eq!(image.num_lit_pixels(), 5486);

    for _ in 0..48 {
        image = image.enhance();
    }

    assert_eq!(image.num_lit_pixels(), 20210);

    Ok(())
}
