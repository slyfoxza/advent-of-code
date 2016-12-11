#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fmt::{self, Display, Formatter};
use std::io::{self, Read};

type BotId = i16;

#[derive(Default)]
struct Bot {
    id: BotId,
    values: [u16; 2],
    low: Option<BotId>,
    high: Option<BotId>
}

impl Display for Bot {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "#{}: {}, {}", self.id, self.values[0], self.values[1])
    }
}

fn find_bot(id: BotId, bots: &Vec<Bot>) -> Result<usize, usize> {
    bots.binary_search_by_key(&id, |b| b.id)
}

fn ensure_bot(id: BotId, bots: &mut Vec<Bot>) -> usize {
    match find_bot(id, bots) {
        Ok(i) => i,
        Err(i) => {
            bots.insert(i, Bot { id: id, ..Default::default() });
            i
        }
    }
}

fn add_value(bots: &mut Vec<Bot>, i: usize, value: u16) -> Result<usize, &str> {
    if bots[i].values[0] == 0 {
        bots[i].values[0] = value;
        Ok(0)
    } else if bots[i].values[1] == 0 {
        if value > bots[i].values[0] {
            bots[i].values[1] = value;
        } else {
            bots[i].values[1] = bots[i].values[0];
            bots[i].values[0] = value;
        }
        Ok(1)
    } else {
        Err("Both values already set")
    }
}

fn try_update(bots: &mut Vec<Bot>, i: usize) {
    if bots[i].values[0] == 0 || bots[i].values[1] == 0 || bots[i].low.is_none()
            || bots[i].high.is_none() {
        return;
    }
    let low_id = bots[i].low.unwrap();
    if low_id >= 0 {
        let low_value = bots[i].values[0];
        if let Ok(low_i) = find_bot(low_id, bots) {
            if let Ok(1) = add_value(bots, low_i, low_value) {
                try_update(bots, low_i);
            }
        }
    }
    let high_id = bots[i].high.unwrap();
    if high_id >= 0 {
        let high_value = bots[i].values[1];
        if let Ok(high_i) = find_bot(high_id, bots) {
            if let Ok(1) = add_value(bots, high_i, high_value) {
                try_update(bots, high_i);
            }
        }
    }
}

fn parse_value_in(input: &str, bots: &mut Vec<Bot>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^value (\d+) goes to bot (\d+)$")
            .unwrap();
    }
    let captures = RE.captures(input).unwrap();
    let id = captures.at(2).map(|v| v.parse::<BotId>().unwrap()).unwrap();
    let i = ensure_bot(id, bots);
    let value = captures.at(1).map(|v| v.parse::<u16>().unwrap()).unwrap();
    if let Ok(1) = add_value(bots, i, value) {
        try_update(bots, i);
    }
}

fn parse_give(input: &str, bots: &mut Vec<Bot>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            "^bot (\\d+) gives low to (bot|output) (\\d+) and high to \
             (bot|output) (\\d+)").unwrap();
    }
    let captures = RE.captures(input).unwrap();
    let id = captures.at(1).map(|v| v.parse::<BotId>().unwrap()).unwrap();
    let mut i = ensure_bot(id, bots);
    let low_id = captures.at(3).map(|v| v.parse::<BotId>().unwrap()).unwrap();
    let low_id = match captures.at(2) {
        Some("bot") => {
            ensure_bot(low_id, bots);
            i = find_bot(id, bots).unwrap();
            low_id
        },
        _ => -low_id
    };
    bots[i].low = Some(low_id);
    let high_id = captures.at(5).map(|v| v.parse::<BotId>().unwrap()).unwrap();
    let high_id = match captures.at(4) {
        Some("bot") => {
            ensure_bot(high_id, bots);
            i = find_bot(id, bots).unwrap();
            high_id
        },
        _ => -high_id
    };
    bots[i].high = Some(high_id);
    try_update(bots, i);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut bots = Vec::<Bot>::new();
    for line in input.lines() {
        if line.starts_with("value ") {
            parse_value_in(line, &mut bots);
        } else {
            parse_give(line, &mut bots);
        }
    }
    for bot in bots {
        println!("{}", bot);
    }
}

#[cfg(test)]
mod tests {
    use super::{Bot, parse_give, parse_value_in};

    #[test]
    fn test1() {
        let mut bots = Vec::<Bot>::new();
        parse_value_in("value 5 goes to bot 2", &mut bots);
        parse_give("bot 2 gives low to bot 1 and high to bot 0", &mut bots);
        parse_value_in("value 3 goes to bot 1", &mut bots);
        parse_give("bot 1 gives low to output 1 and high to bot 0", &mut bots);
        parse_give("bot 0 gives low to output 2 and high to output 0",
                   &mut bots);
        parse_value_in("value 2 goes to bot 2", &mut bots);
        assert_eq!(2, bots[2].values[0]);
        assert_eq!(5, bots[2].values[1]);
    }
}
