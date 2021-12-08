use crate::error;

use itertools::Itertools;
use permutator::copy::Permutation;

#[derive(Debug)]
pub struct Mapper {
    mapping: String,
}

impl Mapper {
    fn from_patterns(patterns: &Vec<String>) -> Result<Self, error::Error> {
        for permutation in vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'].permutation() {
            let permutation_string: String = permutation.iter().collect();
            let mapper = Mapper { mapping: permutation_string };
            if patterns.iter().all(|pattern| mapper.to_digit(pattern).is_some()) {
                return Ok(mapper);
            }
        }
        Err(error::Error::General(format!("failed to find mapping from patterns: {:?}", patterns)))
    }

    fn map_char(&self, c: char) -> char {
        let pos = self.mapping.find(c).expect("could not map char");
        match pos {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            _ => panic!("invalid char"),
        }
    }

    pub fn to_digit(&self, input: &str) -> Option<char> {
        let new: String = input.chars().map(|c| self.map_char(c)).sorted().collect();

        match new.as_ref() {
            "abcefg" => Some('0'),
            "cf" => Some('1'),
            "acdeg" => Some('2'),
            "acdfg" => Some('3'),
            "bcdf" => Some('4'),
            "abdfg" => Some('5'),
            "abdefg" => Some('6'),
            "acf" => Some('7'),
            "abcdefg" => Some('8'),
            "abcdfg" => Some('9'),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    pub patterns: Vec<String>,
    pub output: Vec<String>,
}

impl Entry {
    pub fn output(&self) -> Result<u64, error::Error> {
        let mapper = Mapper::from_patterns(&self.patterns)?;
        let output_string: String = self.output.iter().map(|o| mapper.to_digit(o).unwrap()).collect();
        Ok(output_string.parse()?)
    }
}

impl std::str::FromStr for Entry {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
        let tokens: Vec<&str> = s.split('|').collect();
        if tokens.len() != 2 {
            return Err(error::Error::Parse(format!("invalid Entry: {}", s)));
        }
        Ok(Entry {
            patterns: tokens[0].trim_start().trim_end().split(' ').map(str::to_string).collect(),
            output: tokens[1].trim_start().trim_end().split(' ').map(str::to_string).collect(),
        })
    }
}

#[derive(Debug)]
pub struct Game {
    entries: Vec<Entry>,
}

impl Game {
    pub fn sum(&self) -> u64 {
        self.entries.iter().map(|e| e.output().unwrap()).sum()
    }
}

impl std::str::FromStr for Game {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries: Result<Vec<Entry>, _> = s.lines().map(|line| line.trim_start().trim_end()).filter(|line| !line.is_empty()).map(|line| line.parse()).collect();

        Ok(Game { entries: entries? })
    }
}

impl Game {
    pub fn count_unique_output_values(&self) -> u64 {
        let mut count = 0u64;
        for entry in self.entries.iter() {
            let c = entry.output.iter().filter(|p| p.len() == 4 || p.len() == 2 || p.len() == 3 || p.len() == 7).count();
            //println!("entry: {:?} count: {}", entry, c);
            count += c as u64;
        }
        count
    }
}

#[test]
fn test_mapper() -> Result<(), error::Error> {
    let game: Game = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".parse()?;
    let mapper = Mapper::from_patterns(&game.entries[0].patterns)?;
    assert_eq!(mapper.to_digit("acedgfb").unwrap(), '8');
    assert_eq!(mapper.to_digit("cdfbe").unwrap(), '5');
    assert_eq!(mapper.to_digit("gcdfa").unwrap(), '2');
    assert_eq!(mapper.to_digit("fbcad").unwrap(), '3');
    assert_eq!(mapper.to_digit("dab").unwrap(), '7');
    assert_eq!(mapper.to_digit("cefabd").unwrap(), '9');
    assert_eq!(mapper.to_digit("cdfgeb").unwrap(), '6');
    assert_eq!(mapper.to_digit("eafb").unwrap(), '4');
    assert_eq!(mapper.to_digit("cagedb").unwrap(), '0');
    assert_eq!(mapper.to_digit("ab").unwrap(), '1');
    Ok(())
}

#[test]
fn test_day8() -> Result<(), error::Error> {
    let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let game: Game = input.parse()?;
    assert_eq!(game.entries.len(), 1);
    assert_eq!(game.entries[0].patterns.len(), 10);
    assert_eq!(game.entries[0].output.len(), 4);

    let input = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

    let game: Game = input.parse()?;
    assert_eq!(game.entries.len(), 10);
    assert_eq!(game.count_unique_output_values(), 26);
    assert_eq!(game.entries[0].output()?, 8394);
    assert_eq!(game.entries[9].output()?, 4315);
    assert_eq!(game.sum(), 61229);

    let input = std::fs::read_to_string("input_day8")?;
    let game: Game = input.parse()?;

    assert_eq!(game.entries.len(), 200);
    assert_eq!(game.count_unique_output_values(), 381);
    assert_eq!(game.sum(), 1023686);

    Ok(())
}
