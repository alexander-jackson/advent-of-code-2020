use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Bag<'a> {
    modifier: &'a str,
    colour: &'a str,
}

impl<'a> Bag<'a> {
    pub fn new(modifier: &'a str, colour: &'a str) -> Self {
        Self { modifier, colour }
    }
}

#[derive(Debug)]
struct BagRequirement<'a> {
    count: u32,
    pub bag: Bag<'a>,
}

impl<'a> BagRequirement<'a> {
    pub fn new(count: &str, modifier: &'a str, colour: &'a str) -> Self {
        Self {
            count: u32::from_str(count).unwrap(),
            bag: Bag::new(modifier, colour),
        }
    }
}

fn can_contain<'a>(
    required_contents: &'a HashMap<Bag<'a>, Vec<BagRequirement<'a>>>,
    current: &'a Bag<'a>,
    content: &'a Bag<'a>,
) -> bool {
    if current == content {
        return true;
    }

    // Find the required contents of the current bag
    let contents = &required_contents[current];

    // Check if any of these are the bag we want
    if contents.iter().find(|b| &b.bag == content).is_some() {
        return true;
    }

    // Check if any of them can contain the bag we want
    for required in contents {
        if can_contain(required_contents, &required.bag, &content) {
            return true;
        }
    }

    false
}

fn count_contents<'a>(
    required_contents: &'a HashMap<Bag<'a>, Vec<BagRequirement<'a>>>,
    bag: &'a Bag<'a>,
) -> u32 {
    // We must contain the current bag at least
    let mut count = 1;

    for requirement in &required_contents[bag] {
        count += count_contents(required_contents, &requirement.bag) * requirement.count;
    }

    count
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let rules: Vec<_> = contents.lines().collect();

    let mut required_contents = HashMap::new();

    for rule in &rules {
        let mut tokens = rule.split(' ');

        let lhs = Bag::new(tokens.next().unwrap(), tokens.next().unwrap());

        // Skip the bags and contains
        tokens.next().unwrap();
        tokens.next().unwrap();

        let quantifier = tokens.next().unwrap();

        // Check whether the bag contains nothing
        if quantifier == "no" {
            required_contents.insert(lhs, Vec::new());
            continue;
        }

        // Get the rest of the sentence
        let mut bag_contents = Vec::new();
        bag_contents.push(quantifier);
        bag_contents.extend(tokens);

        let remaining_bags = bag_contents.len() / 4;

        required_contents.insert(
            lhs,
            (0..remaining_bags)
                .map(|i| {
                    let base = i * 4;
                    BagRequirement::new(
                        bag_contents[base],
                        bag_contents[base + 1],
                        bag_contents[base + 2],
                    )
                })
                .collect(),
        );
    }

    let to_find = Bag::new("shiny", "gold");

    let possible: Vec<_> = required_contents
        .keys()
        .filter(|k| can_contain(&required_contents, k, &to_find) && **k != to_find)
        .collect();

    let count = possible.len();

    dbg!(&count);

    let bags_inside = count_contents(&required_contents, &to_find) - 1;
    dbg!(&bags_inside);
}
