use std::io::{self, Read};

#[derive(Debug, PartialEq)]
struct Node {
    x: u8,
    y: u8,
    size: u8,
    used: u8
}

impl Node {
    fn from_df(s: &str) -> Result<Node, ()> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        if parts.len() == 5 {
            let pos_parts = parts[0].split('-').collect::<Vec<_>>();
            if pos_parts.len() == 3 {
                if let Ok(x) = (&pos_parts[1][1..]).parse() {
                    if let Ok(y) = (&pos_parts[2][1..]).parse() {
                        let size = &parts[1][..parts[1].len()-1];
                        if let Ok(size) = size.parse() {
                            let used = &parts[2][..parts[2].len()-1];
                            if let Ok(used) = used.parse() {
                                return Ok(Node{x: x, y: y,
                                               size: size, used: used});
                            }
                        }
                    }
                }
            }
        }
        Err(())
    }

    fn available(&self) -> u8 { self.size - self.used }
    fn is_empty(&self) -> bool { self.used == 0 }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut nodes = Vec::new();
    for line in input.lines() {
        if let Ok(node) = Node::from_df(line.trim()) {
            nodes.push(node);
        }
    }
    let mut viable_pairs = 0;
    for node_a in &nodes {
        for node_b in &nodes {
            if (node_a != node_b) && !node_a.is_empty() && (node_a.used <= node_b.available()) {
                viable_pairs += 1;
            }
        }
    }
    println!("Viable pairs: {}", viable_pairs);
}

#[cfg(test)]
mod test {
    use super::Node;

    #[test]
    fn test_node_from_df() {
        assert_eq!(Ok(Node{x: 4, y: 2, size: 4, used: 3}),
                   Node::from_df("/dev/grid/node-x4-y2  4T  3T  1T 75%"));
    }

    #[test]
    fn test_node_is_empty() {
        assert!(Node{x: 0, y: 0, size: 1, used: 0}.is_empty());
        assert!(!Node{x: 0, y: 0, size: 1, used: 1}.is_empty());
    }

    #[test]
    fn test_node_available() {
        assert_eq!(42, Node{x: 0, y: 0, size: 80, used: 38}.available());
    }
}
