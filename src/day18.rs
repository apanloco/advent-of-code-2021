
use crate::error;

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use permutator::copy::{Combination, Permutation};

#[derive(PartialEq, Debug, Clone)]
pub enum Element {
    Pair(Rc<RefCell<Element>>, Rc<RefCell<Element>>),
    Number(i64),
}

#[derive(PartialEq, Debug)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Number(i64),
    Comma,
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Pair(x, y) => {
                let x = &x.borrow();
                let y = &y.borrow();
                write!(f, "[{},{}]", x, y)
            }
            &Element::Number(number) => {
                write!(f, "{}", number)
            }
        }
    }
}

impl Element {
    pub fn new(s: &str) -> Result<Rc<RefCell<Element>>, error::Error> {
        let pairs = s.trim_start().trim_end()
            .lines()
            .map(|line| {
                let tokens = Element::tokenize(line).unwrap();
                let mut iterator = tokens.iter();

                let lb = iterator.next().unwrap();
                if lb != &Token::LeftBracket {
                    panic!("expected left bracket");
                }

                let pair = Element::parse_pair(&mut iterator).unwrap();

                let rb = iterator.next().unwrap();
                if rb != &Token::RightBracket {
                    panic!("expected right bracket");
                }

                pair
            });

        let mut sum: Option<Rc<RefCell<Element>>> = None;
        for pair in pairs {
            if sum.is_none() {
                sum = Some(Rc::new(RefCell::new(pair)));
            } else {
                let new_element = Element::Pair(sum.unwrap(), Rc::new(RefCell::new(pair)));
                let new_element_rc = Rc::new(RefCell::new(new_element));
                loop {
                    if !Element::explode(new_element_rc.clone()) && !Element::split(new_element_rc.clone()) {
                        break;
                    }
                }
                sum = Some(new_element_rc);
            }
        }

        Ok(sum.unwrap())
    }

    pub fn traverse<F>(element: Rc<RefCell<Element>>, depth: usize, f: &mut F)
        where
            F: FnMut(Rc<RefCell<Element>>, usize) {
        f(element.clone(), depth + 1);
        match element.borrow().deref() {
            Element::Pair(x, y) => {
                Element::traverse(x.clone(), depth + 1, f);
                Element::traverse(y.clone(), depth + 1, f);
            }
            Element::Number(_) => {}
        }
    }

    pub fn split(element: Rc<RefCell<Element>>) -> bool {
        let mut has_split = false;
        Element::traverse(element, 0, &mut |element, _| {
            if !has_split {
                let mut new_element = None;
                if let Element::Number(n) = element.borrow().deref() {
                    if *n >= 10 {
                        let (left, right) = split_number_into_two(*n);
                        new_element = Some(Element::Pair(Rc::new(RefCell::new(Element::Number(left))), Rc::new(RefCell::new(Element::Number(right)))));
                    }
                }
                if let Some(new) = new_element {
                    *element.borrow_mut().deref_mut() = new;
                    has_split = true;
                }
            }
        });

        has_split
    }

    pub fn explode(element: Rc<RefCell<Element>>) -> bool {
        let mut ref_explode = None;
        let mut left_number = None;
        let mut right_number = None;
        let mut ref_left_element = None;
        let mut ref_right_element = None;
        let mut skip_some = 0;

        Element::traverse(element, 0, &mut |element, depth| {
            if ref_explode.is_none() {
                if let Element::Number(_) = element.borrow().deref() {
                    ref_left_element = Some(element.clone());
                }
            }

            if depth >= 5 && ref_explode.is_none() {

                if let Element::Pair(x, y) = element.borrow().deref() {
                    if let Element::Number(n1) = x.borrow().deref() {
                        left_number = Some(*n1);
                    }
                    if let Element::Number(n2) = y.borrow().deref() {
                        right_number = Some(*n2);
                    }

                    if left_number.is_some() && right_number.is_some() {
                        ref_explode = Some(element.clone());
                        skip_some = 3;
                    }
                }
            }

            if ref_explode.is_some() && ref_right_element.is_none() && skip_some == 0 {
                if let Element::Number(_) = element.borrow().deref() {
                    ref_right_element = Some(element.clone());
                }
            }

            if skip_some > 0 {
                skip_some -= 1;
            }
        });


        if ref_explode.is_none() {
            return false;
        }

        *ref_explode.as_ref().unwrap().borrow_mut() = Element::Number(0);
        *ref_explode.as_ref().unwrap().borrow_mut() = Element::Number(0);

        if ref_left_element.is_some() {
            if let Element::Number(ref mut n) = ref_left_element.as_ref().unwrap().borrow_mut().deref_mut() {
                *n += left_number.unwrap();
            }
        }

        if ref_right_element.is_some() {
            if let Element::Number(ref mut n) = ref_right_element.as_ref().unwrap().borrow_mut().deref_mut() {
                *n += right_number.unwrap();
            }
        }

        true
    }

    pub fn tokenize(input: &str) -> Result<Vec<Token>, error::Error> {
        let mut tokens = Vec::new();
        let bytes = input.as_bytes();
        let mut index = 0;
        loop {
            let token = match bytes[index] {
                b'[' => Token::LeftBracket,
                b']' => Token::RightBracket,
                b',' => Token::Comma,
                _ => {
                    let from = index;
                    let mut to = from + 1;
                    loop {
                        if !bytes[to].is_ascii_digit() {
                            break;
                        }
                        to += 1;
                    }
                    index += (to - from) - 1;
                    Token::Number(String::from_utf8_lossy(&bytes[from..to]).parse()?)
                }
            };

            tokens.push(token);

            index += 1;

            if index == bytes.len() {
                break;
            }
        }
        Ok(tokens)
    }

    fn parse_element<'a>(tokens: &mut impl Iterator<Item=&'a Token>) -> Result<Element, error::Error> {
        let token = tokens.next().unwrap();

        let element = match token {
            Token::LeftBracket => {
                let pair = Element::parse_pair(tokens)?;

                let rb = tokens.next().unwrap();
                if rb != &Token::RightBracket {
                    return Err(error::Error::Parse("expected right bracket".to_string()));
                }

                pair
            }
            Token::Number(n) => { Element::Number(n.to_owned()) }
            _ => return Err(error::Error::Parse(format!("invalid token for x: {:?}", token)))
        };

        Ok(element)
    }

    fn parse_pair<'a>(tokens: &mut impl Iterator<Item=&'a Token>) -> Result<Element, error::Error> {
        let x = Element::parse_element(tokens)?;

        let token = tokens.next().unwrap();

        if token != &Token::Comma {
            return Err(error::Error::Parse("expected comma".to_string()));
        }

        let y = Element::parse_element(tokens)?;

        Ok(Element::Pair(Rc::new(RefCell::new(x)), Rc::new(RefCell::new(y))))
    }

    pub fn magnitude_recursive(element: &Element) -> i64 {
        match element {
            Element::Pair(x, y) => {
                3 * Element::magnitude(&x.borrow()) + 2 * Element::magnitude(&y.borrow())
            }
            Element::Number(n) => { *n }
        }
    }

    pub fn magnitude(&self) -> i64 {
        Element::magnitude_recursive(self)
    }
}

impl std::iter::Sum for Element {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.reduce(|acc, elem| Element::Pair(Rc::new(RefCell::new(acc)), Rc::new(RefCell::new(elem)))).unwrap()
    }
}

fn split_number_into_two(number: i64) -> (i64, i64) {
    let left = number / 2;
    let right = number - left;
    (left, right)
}

pub fn find_max_magnitude(input: &str) -> Result<i64, error::Error> {
    let mut lines: Vec<&str> = input.trim_start().trim_end().lines().collect();

    lines.sort_by(|&a, &b| {
        let a = a.matches('[').count();
        let b = b.matches(']').count();
        b.cmp(&a)
    });

    let max_number_of_lines = 22;

    loop {
        lines.pop();

        if lines.len() <= max_number_of_lines {
            break;
        }
    }

    let mut max_magnitude = -1i64;

    for mut combination in lines.combination(2) {
        for permutation in combination.permutation() {
            let input: String = permutation.join("\n");
            let element = Element::new(&input)?;
            let magnitude = element.borrow().magnitude();
            if max_magnitude < magnitude {
                max_magnitude = magnitude;
            }
        }
    }

    Ok(max_magnitude)
}

#[test]
fn test_split_number_into_two() {
    assert_eq!(split_number_into_two(9), (4, 5));
    assert_eq!(split_number_into_two(10), (5, 5));
    assert_eq!(split_number_into_two(11), (5, 6));
}

#[test]
fn test_pair_tokenizer() -> Result<(), error::Error> {
    let tokens = Element::tokenize("[[1111,2222],[[3333,4444],5555]]")?;
    let mut tokens = tokens.iter();
    assert_eq!(tokens.next(), Some(&Token::LeftBracket));
    assert_eq!(tokens.next(), Some(&Token::LeftBracket));
    assert_eq!(tokens.next(), Some(&Token::Number(1111)));
    assert_eq!(tokens.next(), Some(&Token::Comma));
    assert_eq!(tokens.next(), Some(&Token::Number(2222)));
    assert_eq!(tokens.next(), Some(&Token::RightBracket));
    assert_eq!(tokens.next(), Some(&Token::Comma));
    assert_eq!(tokens.next(), Some(&Token::LeftBracket));
    assert_eq!(tokens.next(), Some(&Token::LeftBracket));
    assert_eq!(tokens.next(), Some(&Token::Number(3333)));
    assert_eq!(tokens.next(), Some(&Token::Comma));
    assert_eq!(tokens.next(), Some(&Token::Number(4444)));
    assert_eq!(tokens.next(), Some(&Token::RightBracket));
    assert_eq!(tokens.next(), Some(&Token::Comma));
    assert_eq!(tokens.next(), Some(&Token::Number(5555)));
    assert_eq!(tokens.next(), Some(&Token::RightBracket));
    assert_eq!(tokens.next(), Some(&Token::RightBracket));
    assert_eq!(tokens.next(), None);
    Ok(())
}

#[test]
fn test_magnitude() -> Result<(), error::Error> {
    let pair = Element::new("[9,1]")?;
    assert_eq!(pair.borrow().magnitude(), 29);
    let pair = Element::new("[1,9]")?;
    assert_eq!(pair.borrow().magnitude(), 21);
    let pair = Element::new("[[9,1],[1,9]]")?;
    assert_eq!(pair.borrow().magnitude(), 129);
    let pair = Element::new("[[1,2],[[3,4],5]]")?;
    assert_eq!(pair.borrow().magnitude(), 143);
    let pair = Element::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")?;
    assert_eq!(pair.borrow().magnitude(), 1384);
    let pair = Element::new("[[[[1,1],[2,2]],[3,3]],[4,4]]")?;
    assert_eq!(pair.borrow().magnitude(), 445);
    let pair = Element::new("[[[[3,0],[5,3]],[4,4]],[5,5]]")?;
    assert_eq!(pair.borrow().magnitude(), 791);
    let pair = Element::new("[[[[5,0],[7,4]],[5,5]],[6,6]]")?;
    assert_eq!(pair.borrow().magnitude(), 1137);
    let pair = Element::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")?;
    assert_eq!(pair.borrow().magnitude(), 3488);
    Ok(())
}

#[test]
fn test_display() -> Result<(), error::Error> {
    let pair = Element::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")?;
    assert_eq!(format!("{}", pair.borrow()), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    Ok(())
}

#[test]
fn test_explode() -> Result<(), error::Error> {
    let pair = Element::new(r#"[[[[[9,8],1],2],3],4]"#)?;
    assert!(Element::explode(pair.clone()));
    assert!(!Element::explode(pair.clone()));
    assert_eq!(pair.borrow().to_string(), "[[[[0,9],2],3],4]");

    let pair = Element::new(r#"[7,[6,[5,[4,[3,2]]]]]"#)?;
    assert!(Element::explode(pair.clone()));
    assert!(!Element::explode(pair.clone()));
    assert_eq!(pair.borrow().to_string(), "[7,[6,[5,[7,0]]]]");

    Ok(())
}

#[test]
fn test_day18() -> Result<(), error::Error> {
    let pair = Element::new(r#"
[1,1]
[2,2]
[3,3]
[4,4]"#)?;
    assert_eq!(pair.borrow().to_string(), "[[[[1,1],[2,2]],[3,3]],[4,4]]");

    let pair = Element::new(r#"
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]"#)?;
    assert_eq!(pair.borrow().to_string(), "[[[[3,0],[5,3]],[4,4]],[5,5]]");

    let pair = Element::new(r#"
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]"#)?;
    assert_eq!(pair.borrow().to_string(), "[[[[5,0],[7,4]],[5,5]],[6,6]]");

    let pair = Element::new(r#"
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"#)?;
    assert_eq!(pair.borrow().to_string(), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");

    let pair = Element::new(r#"
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#)?;
    assert_eq!(pair.borrow().to_string(), "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");

    let pair = Element::new(&std::fs::read_to_string("input_day18")?)?;
    assert_eq!(pair.borrow().magnitude(), 3806);

    Ok(())
}

#[test]
fn test_day18_part2() -> Result<(), error::Error> {

    let s = r#"
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

    assert_eq!(find_max_magnitude(s)?, 3993);

    let s = std::fs::read_to_string("input_day18")?;

    assert_eq!(find_max_magnitude(&s)?, 4727);

    Ok(())
}
