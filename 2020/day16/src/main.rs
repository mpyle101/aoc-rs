fn main() {
    let (rules, ticket, nearby) = load(include_str!("./tickets.txt"));

    let ticket_scanning_error_rate = part_one(&rules, &nearby);
    println!("Part 1: {ticket_scanning_error_rate}");

    let departure_product = part_two(&rules, &ticket, &nearby);
    println!("Part 2: {departure_product}");
}

fn part_one(rules: &[Rule], tickets: &[Ticket]) -> u32 {
    tickets.iter().fold(0, |acc, t| {
        let invalid: u32 = t.fields.iter()
            .filter(|&v| rules.iter().all(|r| !valid_field(r, *v))).sum();
        acc + invalid
    })
}

fn part_two(rules: &[Rule], ticket: &Ticket, nearby: &[Ticket]) -> u64 {
    use std::collections::{VecDeque, HashMap, HashSet};

    let tickets = nearby.iter()
        .filter(|t| valid_ticket(t, rules))
        .collect::<Vec<_>>();

    let mut fields = HashMap::new();
    let mut unused = (0..ticket.fields.len()).collect::<HashSet<_>>();
    let mut queue  = rules.iter().collect::<VecDeque<_>>();

    while let Some(rule) = queue.pop_front() {
        let valid = unused.iter()
            .filter(|&n| tickets.iter().all(|&t| valid_field(rule, t.fields[*n])))
            .copied()
            .collect::<Vec<_>>();

        if valid.len() == 1 {
            fields.insert(rule.name, valid[0]);
            unused.remove(&valid[0]);
        } else {
            queue.push_back(rule);
        }
    }

    fields.iter().filter(|(r, _)| r.starts_with("departure"))
        .map(|(_, &i)| ticket.fields[i] as u64)
        .product()
}

fn valid_ticket(ticket: &Ticket, rules: &[Rule]) -> bool {
    ticket.fields.iter().all(|&f| rules.iter().any(|r| valid_field(r, f)))
}

fn valid_field(rule: &Rule, v: u32) -> bool {
    rule.valid.iter().any(|(min, max)| v >= *min && v <= *max)
}

fn load(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut iter = input.split("\n\n");
    let rules = iter.next().unwrap().lines()
        .map(parse_rule)
        .collect();

    let tickets = iter.next().unwrap().lines().skip(1)
        .map(parse_ticket)
        .collect::<Vec<_>>();

    let nearby = iter.next().unwrap().lines().skip(1)
        .map(parse_ticket)
        .collect();

    (rules, tickets.first().cloned().unwrap(), nearby)
}

fn parse_rule(rule: &str) -> Rule {
    let parts = rule.split(": ").collect::<Vec<_>>();
    let valid = parts[1].split(" or ")
        .map(parse_range)
        .collect();

    Rule { name: parts[0], valid }
}

fn parse_range(range: &str) -> (u32, u32) {
    let vals = range.split('-')
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    (vals[0], vals[1])
}

fn parse_ticket(ticket: &str) -> Ticket {
    let fields = ticket.split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    Ticket { fields }
}

#[derive(Debug)]
struct Rule<'a> {
    name: &'a str,
    valid: Vec<(u32, u32)>,
}

#[derive(Clone, Debug)]
struct Ticket {
    fields: Vec<u32>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (rules, ticket, nearby) = load(include_str!("./tickets.txt"));

        let ticket_scanning_error_rate = part_one(&rules, &nearby);
        assert_eq!(ticket_scanning_error_rate, 25059);

        let departure_product = part_two(&rules, &ticket, &nearby);
        assert_eq!(departure_product, 3253972369789);
    }
}