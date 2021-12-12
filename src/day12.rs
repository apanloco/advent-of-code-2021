use crate::error;

use std::collections::HashMap;

pub enum GraphRules {
    FirstPart,
    SecondPart,
}

pub struct Graph {
    connection_map: HashMap<String, Vec<String>>,
}

#[derive(Clone)]
pub struct Path {
    path: String,
    node_counter: HashMap<String, usize>,
    any_small_duplicates: bool,
}

impl std::str::FromStr for Graph {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut connection_map: HashMap<String, Vec<String>> = HashMap::new();

        let connection_iter = s.lines().filter(|line| !line.trim_start().trim_end().is_empty()).map(|line| {
            let mut tokens = line.split('-');
            (tokens.next().unwrap().to_owned(), tokens.next().unwrap().to_owned())
        });

        for (from, to) in connection_iter {
            connection_map.entry(from.to_string()).or_default().push(to.to_string());
            connection_map.entry(to).or_default().push(from);
        }

        Ok(Graph { connection_map })
    }
}

pub fn is_small_cave(node: &str) -> bool {
    node != "start" && node != "end" && node.chars().all(|c| c.is_lowercase())
}

impl Graph {
    pub fn generate_paths(&self, rules: GraphRules) -> Vec<String> {
        let mut building_paths: Vec<Path> = vec!["start".parse().unwrap()];
        let mut completed_paths: Vec<Path> = vec![];

        loop {
            let mut new_paths: Vec<Path> = vec![];

            for path in &building_paths {
                let to_nodes = self.connection_map.get(path.last_node()).unwrap();

                for to_node in to_nodes {
                    if !path.can_add(to_node, &rules) {
                        continue;
                    }

                    let mut new_path = path.clone();
                    new_path.add_node(to_node);

                    if to_node == "end" {
                        completed_paths.push(new_path)
                    } else {
                        new_paths.push(new_path);
                    }
                }
            }

            if new_paths.is_empty() {
                break;
            }

            building_paths = new_paths;
        }

        completed_paths.into_iter().map(|p| p.path).collect()
    }
}

impl std::str::FromStr for Path {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut path: Path = Path {
            path: "".to_string(),
            node_counter: Default::default(),
            any_small_duplicates: false,
        };
        for node in s.split(',') {
            path.add_node(node);
        }
        Ok(path)
    }
}

impl Path {
    fn can_add(&self, node: &str, rules: &GraphRules) -> bool {
        if node == "start" {
            return false;
        }

        if !is_small_cave(node) {
            return true;
        }

        match rules {
            GraphRules::FirstPart => !self.path.contains(&node.to_string()),
            GraphRules::SecondPart => {
                if !self.path.contains(node) {
                    return true;
                }
                !self.any_small_duplicates
            }
        }
    }

    fn add_node(&mut self, node: &str) {
        if !self.path.is_empty() {
            self.path.push(',');
        }
        self.path += node;
        if !self.any_small_duplicates && is_small_cave(node) {
            let entry = self.node_counter.entry(node.to_string()).or_default();
            if *entry > 0 {
                self.any_small_duplicates = true;
            } else {
                *entry += 1
            }
        }
    }

    fn last_node(&self) -> &str {
        match self.path.rfind(',') {
            None => &self.path,
            Some(pos) => &self.path[pos + 1..],
        }
    }
}

#[test]
fn test_utils() -> Result<(), error::Error> {
    assert!(is_small_cave(""));
    assert!(!is_small_cave("Asd"));
    assert!(is_small_cave("a"));
    Ok(())
}

#[test]
fn test_path_last_node() -> Result<(), error::Error> {
    let path: Path = "".parse()?;
    assert_eq!(path.last_node(), "");
    let path: Path = "start".parse()?;
    assert_eq!(path.last_node(), "start");
    let path: Path = "start,a".parse()?;
    assert_eq!(path.last_node(), "a");
    Ok(())
}

#[test]
fn test_path_can_add() -> Result<(), error::Error> {
    let path: Path = "A,a".parse()?;
    assert!(!path.can_add("a", &GraphRules::FirstPart));
    assert!(path.can_add("c", &GraphRules::FirstPart));
    assert!(path.can_add("A", &GraphRules::FirstPart));
    assert!(path.can_add("a", &GraphRules::SecondPart));
    assert!(path.can_add("c", &GraphRules::SecondPart));
    assert!(path.can_add("A", &GraphRules::SecondPart));

    let path: Path = "A,a,a".parse()?;
    assert!(path.can_add("c", &GraphRules::FirstPart));
    assert!(!path.can_add("a", &GraphRules::FirstPart));
    assert!(path.can_add("A", &GraphRules::FirstPart));
    assert!(path.can_add("c", &GraphRules::SecondPart));
    assert!(!path.can_add("a", &GraphRules::SecondPart));
    assert!(path.can_add("A", &GraphRules::SecondPart));

    let path: Path = "A,a,a,c".parse()?;
    assert!(!path.can_add("c", &GraphRules::FirstPart));
    assert!(!path.can_add("a", &GraphRules::FirstPart));
    assert!(path.can_add("A", &GraphRules::FirstPart));
    assert!(!path.can_add("c", &GraphRules::SecondPart));
    assert!(!path.can_add("a", &GraphRules::SecondPart));
    assert!(path.can_add("A", &GraphRules::SecondPart));

    Ok(())
}

#[test]
fn test_day12() -> Result<(), error::Error> {
    let graph: Graph = r#"
start-A
start-b
A-c
A-b
b-d
A-end
b-end"#
        .parse()?;
    assert_eq!(graph.generate_paths(GraphRules::FirstPart).len(), 10);
    assert_eq!(graph.generate_paths(GraphRules::SecondPart).len(), 36);

    let graph: Graph = r#"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#
        .parse()?;
    assert_eq!(graph.generate_paths(GraphRules::FirstPart).len(), 19);
    assert_eq!(graph.generate_paths(GraphRules::SecondPart).len(), 103);

    let graph: Graph = r#"
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#
        .parse()?;
    assert_eq!(graph.generate_paths(GraphRules::FirstPart).len(), 226);
    assert_eq!(graph.generate_paths(GraphRules::SecondPart).len(), 3509);

    let graph: Graph = std::fs::read_to_string("input_day12")?.parse()?;
    assert_eq!(graph.generate_paths(GraphRules::FirstPart).len(), 5252);
    assert_eq!(graph.generate_paths(GraphRules::SecondPart).len(), 147784);

    Ok(())
}
