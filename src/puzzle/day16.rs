/*
** src/puzzle/day16.rs
** https://adventofcode.com/2020/day/16
*/

use std::collections::HashMap;

use crate::puzzle::*;
use crate::utils::input_to_lines;

const INPUT: &str = include_str!("../../input/16.input");

const N_FIELDS: usize = 20;

struct TicketField<'a> {
    name: &'a str,
    range_1: (u16, u16),
    range_2: (u16, u16),
}

impl<'a> TicketField<'a> {
    fn is_valid(&self, value: u16) -> bool {
        let (a, b) = self.range_1;
        let (c, d) = self.range_2;
        (value >= a && value <= b) || (value >= c && value <= d)
    }
}

impl<'a> From<&'a str> for TicketField<'a> {
    fn from(s: &'a str) -> Self {
        split_into!(s, ": ", name, ranges);
        split_into!(ranges, " or ", range_1_str, range_2_str);

        let range_1 = match split!(range_1_str, '-') {
            [start, end] => (start.parse().unwrap(), end.parse().unwrap()),
            _ => unreachable!(),
        };

        let range_2 = match split!(range_2_str, '-') {
            [start, end] => (start.parse().unwrap(), end.parse().unwrap()),
            _ => unreachable!(),
        };

        Self {
            name,
            range_1,
            range_2,
        }
    }
}

struct Ticket {
    fields: Vec<u16>,
}

impl From<&str> for Ticket {
    fn from(s: &str) -> Self {
        let fields = s.split(',').map(|s| s.parse().unwrap()).collect();
        Self { fields }
    }
}

pub struct Day16<'a> {
    fields: Vec<TicketField<'a>>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl<'a> Day16<'a> {
    pub fn new() -> Self {
        split_into!(INPUT, "\n\n", fields_str, ticket_str, nearby_str);

        let fields = input_to_lines(fields_str).map(TicketField::from).collect();
        let my_ticket = Ticket::from(input_to_lines(ticket_str).nth(1).unwrap());
        let nearby_tickets = input_to_lines(nearby_str)
            .skip(1)
            .map(Ticket::from)
            .collect();

        Self {
            fields,
            my_ticket,
            nearby_tickets,
        }
    }

    fn valid_for_any_field(&self, value: u16) -> bool {
        self.fields.iter().any(|f| f.is_valid(value))
    }
}

impl<'a> Puzzle for Day16<'a> {
    // Consider the validity of the nearby tickets you scanned. What is your
    // ticket scanning error rate?
    fn part1(&self) -> Result<Solution> {
        let mut error_rate = 0;
        for ticket in self.nearby_tickets.iter() {
            error_rate += ticket
                .fields
                .iter()
                .filter(|&&f| !self.valid_for_any_field(f))
                .sum::<u16>() as u64;
        }

        Ok(error_rate.into())
    }

    // Once you work out which field is which, look for the six fields on your
    // ticket that start with the word departure. What do you get if you
    // multiply those six values together?
    fn part2(&self) -> Result<Solution> {
        // disregard any ticket with invalid fields
        let valid_tickets = self
            .nearby_tickets
            .iter()
            .filter(|t| t.fields.iter().all(|&f| self.valid_for_any_field(f)))
            .collect::<Vec<_>>();

        //
        // determine the field names
        //

        // note: there is not a clean one-to-one mapping; do an initial pass to
        // assign all possibilities
        let mut field_names = HashMap::new();
        for field in self.fields.iter() {
            let mut valid = Vec::with_capacity(N_FIELDS);
            for nf in 0..N_FIELDS {
                if valid_tickets.iter().all(|t| field.is_valid(t.fields[nf])) {
                    valid.push(nf);
                }
            }
            field_names.insert(field.name, valid);
        }

        // now we can greedily assign names to the fields: there should be one
        // field with only a single possibility - assign it and remove from all
        // other field possibilities; there should now be another field with
        // only a single possibility, and this chain will continue until all
        // fields have been assigned
        let mut field_names_final = [""; N_FIELDS];
        for _ in 0..self.fields.len() {
            // find the field with a single possibility
            let (field_name, field_index) = field_names.iter().find(|(_, v)| v.len() == 1).unwrap();
            let (field_name, field_index) = (*field_name, field_index[0]);
            field_names_final[field_index] = field_name;
            // remove as a possibility from other fields
            for (_, possible_fields) in field_names.iter_mut() {
                if possible_fields.contains(&field_index) {
                    let i = possible_fields
                        .iter()
                        .position(|&x| x == field_index)
                        .unwrap();
                    possible_fields.remove(i);
                }
            }
        }

        let solution = self
            .my_ticket
            .fields
            .iter()
            .zip(field_names_final.iter())
            .filter(|(_, fname)| fname.starts_with("departure"))
            .fold(1u64, |acc, (&field, _)| acc * field as u64);

        Ok(solution.into())
    }
}
