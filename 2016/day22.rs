use std::cmp;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
struct Node {
    x: u8,
    y: u8,
    size: u16,
    used: u16
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

    fn available(&self) -> u16 { self.size - self.used }
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
    let mut width: usize = 0;
    let mut height: usize = 0;
    for node_a in &nodes {
        width = cmp::max(width, node_a.x as usize);
        height = cmp::max(height, node_a.y as usize);
        for node_b in &nodes {
            if (node_a != node_b) && !node_a.is_empty() && (node_a.used <= node_b.available()) {
                viable_pairs += 1;
            }
        }
    }
    width += 1;
    height += 1;
    println!("Viable pairs: {}", viable_pairs);
    let mut grid = vec![1; width * height];
    let mut empty: (usize, usize) = (0, 0);
    for node in nodes {
        let i = node.y as usize * width + node.x as usize;
        if node.used == 0 {
            grid[i] = 0;
            empty = (node.x as usize, node.y as usize);
        }
        else if node.size >= 500 { grid[i] = 2; }
    }
    grid[width - 1] = 4;

    let mut steps = 0;
    while (empty.1 != 0) || (empty.0 != width - 2) {
        grid[empty.1 * width + empty.0] = 1;
        if empty.1 != 0 {
            if grid[(empty.1 - 1) * width + empty.0] == 2 {
                empty = (empty.0 - 1, empty.1);
            } else {
                empty = (empty.0, empty.1 - 1);
            }
        } else {
            empty = (empty.0 + 1, empty.1);
        }
        grid[empty.1 * width + empty.0] = 0;
        steps += 1;
    }
    println!("{} steps to reach goal", steps);
    let mut goal: (usize, usize) = (width - 1, 0);
    while (goal.0 != 0) || (goal.1 != 0) {
        grid[empty.1 * width + empty.0] = 1;
        if empty.1 == goal.1 && empty.0 < goal.0 {
            let tmp = goal;
            goal = empty;
            empty = tmp;
        } else if empty.0 < goal.0 {
            if empty.1 < goal.1 {
                empty.1 += 1;
            } else {
                empty.1 -= 1;
            }
        } else if empty.1 == goal.1 {
            if empty.1 == 0 {
                empty.1 += 1;
            } else {
                empty.1 -= 1;
            }
        } else {
            if empty.0 == 0 {
                break;
            }
            empty.0 -= 1;
        }
        grid[empty.1 * width + empty.0] = 0;
        grid[goal.1 * width + goal.0] = 4;
        steps += 1;
    }

    println!("Steps: {}", steps);
    let mut x = 0;
    for cell in grid {
        if x == 0 { print!("\n"); }
        if cell == 0 { print!("_"); }
        else if cell == 2 { print!("#"); }
        else if cell == 4 { print!("G"); }
        else { print!("."); }
        x += 1;
        if x == width {
            x = 0;
        }
    }
    print!("\n");
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
