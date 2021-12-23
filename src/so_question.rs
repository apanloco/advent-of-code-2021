// https://stackoverflow.com/a/70481871/592463

#[derive(Debug)]
pub enum NodeType {
    None,
    Node(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    next: NodeType,
}

impl Node {
    pub fn traverse_recursively<'s, F>(&'s self, depth: usize, f: &mut F)
    where
        F: FnMut(&'s Node, usize),
    {
        f(self, depth);

        match &self.next {
            NodeType::None => {}
            NodeType::Node(node) => {
                node.traverse_recursively(depth + 1, f);
            }
        }
    }

    pub fn visit_all<'s, F>(&'s self, f: &mut F)
    where
        F: FnMut(&'s Node, usize),
    {
        self.traverse_recursively(1, f);
    }
}

pub fn create_small_recursive_structure() -> Node {
    Node {
        next: NodeType::Node(Box::new(Node {
            next: NodeType::Node(Box::new(Node { next: NodeType::None })),
        })),
    }
}

#[test]
fn test_so() {
    let parent = create_small_recursive_structure();

    let mut visited = Vec::new();

    parent.visit_all(&mut |node, depth| {
        visited.push((node, depth));
    });

    for node in visited {
        println!("{:?}", &node);
    }
}
