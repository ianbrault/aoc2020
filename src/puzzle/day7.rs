/*
** src/puzzle/day7.rs
** https://adventofcode.com/2020/day/7
*/

use std::collections::{BTreeSet, HashMap};

use crate::puzzle::*;
use crate::utils::input_to_lines;

const INPUT: &str = include_str!("../../input/7.input");

struct Rule {
    bag: &'static str,
    contains: Vec<(u8, &'static str)>,
}

impl Rule {
    fn parse_contained_bag(bag: &str) -> (u8, &str) {
        // note: number of bags is guaranteed to be a single digit
        let n = bag[0..1].parse().unwrap();

        let contained_bag = if n == 1 {
            &bag[2..(bag.len() - 4)]
        } else {
            &bag[2..(bag.len() - 5)]
        };

        (n, contained_bag)
    }
}

impl From<&'static str> for Rule {
    fn from(s: &'static str) -> Self {
        // ignore the trailing period
        let rule = &s[0..(s.len() - 1)];

        let (bag, contains_str) = match split!(rule, " contain ") {
            [bag, contains_str] => (bag.strip_suffix(" bags").unwrap(), *contains_str),
            _ => unreachable!(),
        };

        // if there are bags contained within, split and parse
        let contains = if contains_str == "no other bags" {
            vec![]
        } else {
            contains_str
                .split(", ")
                .map(Self::parse_contained_bag)
                .collect()
        };

        Self {
            bag,
            contains,
        }
    }
}

struct BagSolver1<'a> {
    // memoize bags which can contain at least 1 shiny gold bag
    solved_set: BTreeSet<&'a str>,
    // used to store intermediate results i.e. a green bag contains red & blue
    // bags but we do not know if either can contain a gold bag
    // note: this uses a reverse linkage e.g. for the above, the rule would
    // lead to 2 holding cell entries, red->green and blue->green
    // note: the second element is a list since there is a many-to-one
    // containment relationship
    holding_cell: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> BagSolver1<'a> {
    fn new() -> Self {
        Self {
            solved_set: BTreeSet::new(),
            holding_cell: HashMap::new(),
        }
    }

    fn process_rule(&mut self, rule: &Rule) {
        // check if the bag contains a shiny gold bag
        let contains_gold = rule.contains
            .iter()
            .any(|(_, b)| *b == "shiny gold");
        // also check if any of the contained bags are in the solved set
        let contains_solved = rule.contains
            .iter()
            .any(|(_, b)| self.solved_set.contains(b));

        if contains_gold || contains_solved {
            self.found_solution(rule.bag);
        } else {
            // otherwise, add the rule to the holding cell
            for (_, contained) in rule.contains.iter() {
                let cell = self.holding_cell.entry(contained).or_insert(vec![]);
                cell.push(rule.bag);
            }
        }
    }

    fn found_solution(&mut self, bag: &'a str) {
        // if the bag is or contains a solution, add it to the solved set
        self.solved_set.insert(bag);
        // then check the holding cell for anything which can be solved using
        // our new solution
        if let Some(new_solutions) = self.holding_cell.remove(bag) {
            for new_solution in new_solutions.into_iter() {
                // need to recurse on these new solutions
                self.found_solution(new_solution);
            }
        }
    }
}

struct BagSolver2<'a> {
    // place all rules in a map for quick lookups
    rule_map: HashMap<&'a str, &'a Vec<(u8, &'a str)>>,
}

impl<'a> BagSolver2<'a> {
    fn new(rules: impl Iterator<Item = &'a Rule>) -> Self {
        let rule_map = rules
            .map(|r| (r.bag, &r.contains))
            .collect();

        Self {
            rule_map,
        }
    }

    fn count_contained_bags(&self, bag: &'a str) -> u64 {
        let contained = self.rule_map.get(bag).unwrap();

        // recurse on any contained bags
        // we could improve by memoizing results, in case different branches of
        // the tree have the same bags, but it is much simpler to map and sum
        // as below, and is still reasonably fast
        contained
            .iter()
            // include 1 for the current bag
            .map(|(n, bag)| (*n as u64) * (1 + self.count_contained_bags(bag)))
            .sum()
    }
}

pub struct Day7 {
    // each rule is a tuple with:
    // (1) the bag
    // (2) a list of bags that can be contained within
    rules: Vec<Rule>,
}

impl Day7 {
    pub fn new() -> Self {
        let rules = input_to_lines(INPUT)
            .map(Rule::from)
            .collect();

        Self {
            rules
        }
    }
}

impl Puzzle for Day7 {
    // How many bag colors can eventually contain at least one shiny gold bag?
    fn part1(&self) -> Result<Solution> {
        // use the BagSolver1, documented above
        let mut solver = BagSolver1::new();

        for rule in self.rules.iter() {
            solver.process_rule(rule);
        }

        Ok((solver.solved_set.len() as u64).into())
    }

    // How many individual bags are required inside your single shiny gold bag?
    fn part2(&self) -> Result<Solution> {
        // use the BagSolver2, documented above
        let solver = BagSolver2::new(self.rules.iter());

        Ok(solver.count_contained_bags("shiny gold").into())
    }
}
