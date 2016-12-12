use std::cmp::{Eq, PartialEq};
use std::collections::{HashSet, LinkedList};
use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Hash)]
struct Floor {
    generators: Vec<bool>,
    microchips: Vec<bool>
}

impl Eq for Floor {}

impl PartialEq for Floor {
    fn eq(&self, other: &Floor) -> bool {
        self.generators.iter().enumerate()
            .all(|g| other.generators[g.0] == *g.1)
            && self.microchips.iter().enumerate()
            .all(|m| other.microchips[m.0] == *m.1)
    }
}

#[derive(Debug)]
struct State {
    depth: u8,
    elevator: u8,
    floors: [Floor; 4]
}

impl Clone for State {
    fn clone(&self) -> State {
        State {
            depth: self.depth,
            elevator: self.elevator,
            floors: [
                self.floors[0].clone(),
                self.floors[1].clone(),
                self.floors[2].clone(),
                self.floors[3].clone()
            ]
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:3} @ {}", self.depth, self.elevator);
        for j in 0..4 {
            let fl = &self.floors[3 - j];
            write!(f, "\n");
            for i in 0..fl.generators.len() {
                if fl.generators[i] {
                    write!(f, "G");
                } else {
                    write!(f, ".");
                }
                if fl.microchips[i] {
                    write!(f, "M");
                } else {
                    write!(f, ".");
                }
            }
        }
        Ok(())
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elevator.hash(state);
        self.floors.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.elevator == other.elevator
            && self.floors.iter().enumerate()
            .all(|f| *f.1 == other.floors[f.0])
    }
}

impl State {
    fn is_win(&self) -> bool {
        self.floors[3].generators.iter().all(|g| *g)
            && self.floors[3].microchips.iter().all(|m| *m)
    }

    fn is_valid(&self) -> bool {
        self.floors.iter().all(|floor| {
            floor.generators.iter()
                .all(|g| !g) || floor.microchips.iter().enumerate()
                .filter(|m| *m.1).all(|m| floor.generators[m.0])
        })
    }

    fn new_move(&self, elevator_dst: u8, gen1: Option<usize>,
                gen2: Option<usize>, mc1: Option<usize>,
                mc2: Option<usize>) -> Result<State, usize> {
        if !((gen1.is_some() && gen2.is_some())
             || (gen1.is_some() && mc1.is_some())
             || (mc1.is_some() && mc2.is_some())) {
            return Err(0);
        }
        let mut floors = [
            self.floors[0].clone(),
            self.floors[1].clone(),
            self.floors[2].clone(),
            self.floors[3].clone()
        ];
        if let Some(g) = gen1 {
            if self.floors[self.elevator as usize].generators[g] {
                floors[self.elevator as usize].generators[g] = false;
                floors[elevator_dst as usize].generators[g] = true;
            }
        }
        if let Some(g) = gen2 {
            if self.floors[self.elevator as usize].generators[g] {
                floors[self.elevator as usize].generators[g] = false;
                floors[elevator_dst as usize].generators[g] = true;
            }
        }
        if let Some(m) = mc1 {
            if self.floors[self.elevator as usize].microchips[m] {
                floors[self.elevator as usize].microchips[m] = false;
                floors[elevator_dst as usize].microchips[m] = true;
            }
        }
        if let Some(m) = mc2 {
            if self.floors[self.elevator as usize].microchips[m] {
                floors[self.elevator as usize].microchips[m] = false;
                floors[elevator_dst as usize].microchips[m] = true;
            }
        }
        let state = State {
            depth: self.depth + 1,
            elevator: elevator_dst,
            floors: floors
        };
        if state.is_valid() && !self.floors.iter().enumerate()
            .all(|f| *f.1 == state.floors[f.0]) {
            Ok(state)
        } else {
            Err(1)
        }
    }
}

macro_rules! try_insert {
    ($s:expr, $q:expr, $st:expr, $ed:expr, $g1:expr, $g2:expr, $m1:expr,
     $m2:expr) => {
        if let Ok(new_state) = $st.new_move($ed, $g1, $g2, $m1, $m2) {
            if $s.insert(new_state.clone()) {
                $q.push_back(new_state);
            }
        }
    }
}

fn find_win(initial_state: State) -> Result<u8, u8> {
    let mut queue = LinkedList::<State>::new();
    let mut seen = HashSet::<State>::new();
    queue.push_back(initial_state.clone());
    seen.insert(initial_state.clone());
    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        // println!("Q@{:2} {}\n", queue.len(), state);
        if state.is_win() {
            return Ok(state.depth);
        }
        let n_elements = state.floors[0].generators.len();
        for elevator_dst in 0..4 {
            let diff = elevator_dst as i8 - state.elevator as i8;
            if diff != -1 && diff != 1 {
                continue;
            }
            for i in 0..n_elements {
                for j in 0..n_elements {
                    try_insert!(seen, queue, state, elevator_dst, Some(i), None,
                                None, None);
                    try_insert!(seen, queue, state, elevator_dst, None, None,
                                Some(i), None);
                    try_insert!(seen, queue, state, elevator_dst, Some(i), None,
                                Some(j), None);
                    if i != j {
                        try_insert!(seen, queue, state, elevator_dst, Some(i),
                                    Some(j), None, None);
                        try_insert!(seen, queue, state, elevator_dst, None,
                                    None, Some(i), Some(j));
                    }
                }
            }
        }
    }
    Err(0)
}

fn main() {
    let initial_state = State {
        depth: 0,
        elevator: 0,
        floors: [
            Floor { generators: vec![true, true, true, true, true],
                    microchips: vec![false, true, false, true, true] },
            Floor { generators: vec![false, false, false, false, false],
                    microchips: vec![true, false, true, false, false] },
            Floor { generators: vec![false, false, false, false, false],
                    microchips: vec![false, false, false, false, false] },
            Floor { generators: vec![false, false, false, false, false],
                    microchips: vec![false, false, false, false, false] },
        ]
    };
    if let Ok(steps) = find_win(initial_state) {
        println!("{}", steps);
    } else {
        println!("rekt");
    }
}

#[cfg(test)]
mod tests {
    use super::{Floor, State};

    #[test]
    fn test() {
        let initial_state = State {
            depth: 0,
            elevator: 0,
            floors: [
                Floor { generators: vec![false, false],
                        microchips: vec![true, true] },
                Floor { generators: vec![true, false],
                        microchips: vec![false, false] },
                Floor { generators: vec![false, true],
                        microchips: vec![false, false] },
                Floor { generators: vec![false, false],
                        microchips: vec![false, false] }
            ]
        };
        assert!(initial_state.is_valid());
        assert_eq!(Ok(11), super::find_win(initial_state));
    }

    #[test]
    fn test2() {
        let state = State {
            depth: 1,
            elevator: 1,
            floors: [
                Floor { generators: vec![false, false],
                        microchips: vec![false, true] },
                Floor { generators: vec![true, false],
                        microchips: vec![true, false] },
                Floor { generators: vec![false, true],
                        microchips: vec![false, false] },
                Floor { generators: vec![false, false],
                        microchips: vec![false, false] }
            ]
        };
        assert!(state.is_valid());
    }

    #[test]
    fn test_cmp() {
        let state1 = State {
            depth: 0,
            elevator: 0,
            floors: [
                Floor { generators: vec![false], microchips: vec![true] },
                Floor { generators: vec![false], microchips: vec![true] },
                Floor { generators: vec![false], microchips: vec![true] },
                Floor { generators: vec![false], microchips: vec![true] }
            ]
        };
        let state2 = State {
            depth: 3,
            elevator: 0,
            floors: [
                Floor { generators: vec![false], microchips: vec![true] },
                Floor { generators: vec![false], microchips: vec![true] },
                Floor { generators: vec![false], microchips: vec![true] },
                Floor { generators: vec![false], microchips: vec![true] }
            ]
        };
        assert_eq!(state1, state2);
    }
}
