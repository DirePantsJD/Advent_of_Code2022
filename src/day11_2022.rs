use std::vec;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: (char, u64),
    test: u64,
    if_false: usize,
    if_true: usize,
    inspections: u64,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            items: vec![],
            operation: (' ', 0),
            test: 0,
            if_false: 0,
            if_true: 0,
            inspections: 0,
        }
    }

    fn handle_item(&self, mut item: u64) -> (usize, u64) {
        match self.operation.0 {
            '*' => {
                if self.operation.1 != 0 {
                    item *= self.operation.1;
                } else {
                    item *= item;
                }
            }
            '+' => item += self.operation.1,
            _ => (),
        }
        item = div_floor(item, 3);
        if item % self.test == 0 {
            (self.if_true, item)
        } else {
            (self.if_false, item)
        }
    }

    fn handle_item2(&self, mut item: u64, mmc: u64) -> (usize, u64) {
        match self.operation.0 {
            '*' => {
                if self.operation.1 != 0 {
                    item *= self.operation.1;
                } else {
                    item *= item;
                }
            }
            '+' => item += self.operation.1,
            _ => (),
        }
        item = item % mmc;
        if item % self.test == 0 {
            (self.if_true, item)
        } else {
            (self.if_false, item)
        }
    }
}

pub fn sum_of_two_max_monkey_inspections(input: Vec<String>) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut id: usize = 0;
    for line in input {
        let mut words = line.trim().split(" ");

        match words.next().unwrap() {
            "Monkey" => {
                id = words
                    .next()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as usize;
                monkeys.push(Monkey::new());
            }
            "Starting" => {
                words.next();
                for item_comma in words {
                    let item = item_comma.split(",").next().unwrap();
                    monkeys[id].items.push(item.parse::<u64>().unwrap());
                }
            }
            "Operation:" => {
                monkeys[id].operation = (
                    words.nth(3).unwrap().chars().next().unwrap(),
                    words.next().unwrap().parse().unwrap_or(0),
                )
            }
            "Test:" => monkeys[id].test = words.nth(2).unwrap().parse().unwrap(),
            "If" => match words.next().unwrap() {
                "true:" => monkeys[id].if_true = words.nth(3).unwrap().parse().unwrap(),
                "false:" => monkeys[id].if_false = words.nth(3).unwrap().parse().unwrap(),
                _ => (),
            },
            _ => (),
        }
    }

    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            let items: Vec<u64> = monkeys[idx].items.drain(..).collect();
            for item in items {
                let (receiving, item) = monkeys[idx].handle_item(item);
                monkeys[idx].inspections += 1;
                monkeys[receiving].items.push(item);
            }
        }
    }
    let (mnk_b1, mnk_b2) = get_two_max(&monkeys);
    mnk_b1 * mnk_b2
}
pub fn sum_of_two_max_monkey_inspections2(input: Vec<String>) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut id: usize = 0;
    for line in input {
        let mut words = line.trim().split(" ");

        match words.next().unwrap() {
            "Monkey" => {
                id = words
                    .next()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as usize;
                monkeys.push(Monkey::new());
            }
            "Starting" => {
                words.next();
                for item_comma in words {
                    let item = item_comma.split(",").next().unwrap();
                    monkeys[id].items.push(item.parse::<u64>().unwrap());
                }
            }
            "Operation:" => {
                monkeys[id].operation = (
                    words.nth(3).unwrap().chars().next().unwrap(),
                    words.next().unwrap().parse().unwrap_or(0),
                )
            }
            "Test:" => monkeys[id].test = words.nth(2).unwrap().parse().unwrap(),
            "If" => match words.next().unwrap() {
                "true:" => monkeys[id].if_true = words.nth(3).unwrap().parse().unwrap(),
                "false:" => monkeys[id].if_false = words.nth(3).unwrap().parse().unwrap(),
                _ => (),
            },
            _ => (),
        }
    }

    let mmc = monkeys.iter().fold(1, |acc, mnk| acc * mnk.test);
    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            let items: Vec<u64> = monkeys[idx].items.drain(..).collect();
            for item in items {
                let (receiving, item) = monkeys[idx].handle_item2(item, mmc);
                monkeys[idx].inspections += 1;
                monkeys[receiving].items.push(item);
            }
        }
    }
    let (mnk_b1, mnk_b2) = get_two_max(&monkeys);
    mnk_b1 * mnk_b2
}

fn div_floor(n1: u64, n2: u64) -> u64 {
    let div = n1 / n2;
    let remainder = n1 % n2;

    if remainder < 5 {
        return div;
    }
    div - 1
}

fn get_two_max(monkeys: &Vec<Monkey>) -> (u64, u64) {
    let mut biggest: u64 = 0;
    let mut snd_biggest: u64 = 0;
    for idx in 0..monkeys.len() {
        let tmp = monkeys[idx].inspections;
        if idx == 0 {
            biggest = tmp;
            continue;
        }
        if tmp > biggest {
            snd_biggest = biggest;
            biggest = tmp;
        } else if tmp > snd_biggest {
            snd_biggest = tmp;
        }
    }
    (biggest, snd_biggest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_item() {
        let mnk = Monkey {
            items: vec![83, 97, 95, 51],
            operation: ('*', 0),
            test: 17,
            if_true: 1,
            if_false: 2,
            inspections: 0,
        };
        // assert_eq!(mnk.handle_item(83),(2,525));
        // assert_eq!(mnk.handle_item(97),(2,614));
        assert_eq!(mnk.handle_item(95), (2, 3008));
        // assert_eq!(mnk.handle_item(51),(1,867));
    }

    #[test]
    fn test_two_biggest() {
        let input = vec![
            Monkey {
                items: vec![83, 97, 95, 67],
                operation: ('*', 19),
                test: 17,
                if_true: 1,
                if_false: 2,
                inspections: 20,
            },
            Monkey {
                items: vec![71, 70, 79, 88, 56, 70],
                operation: ('+', 2),
                test: 19,
                if_true: 2,
                if_false: 0,
                inspections: 10,
            },
            Monkey {
                items: vec![98, 51, 51, 63, 80, 85, 84, 95],
                operation: ('+', 7),
                test: 7,
                if_true: 0,
                if_false: 1,
                inspections: 15,
            },
        ];
        assert_eq!(get_two_max(&input), (20, 15));
    }

    #[test]
    fn test_monkey_b() {
        let input = vec![
            "Monkey 0:".to_string(),
            "  Starting items: 79, 98".to_string(),
            "  Operation: new = old * 19".to_string(),
            "  Test: divisible by 23".to_string(),
            "    If true: throw to monkey 2".to_string(),
            "    If false: throw to monkey 3".to_string(),
            "".to_string(),
            "Monkey 1:".to_string(),
            "  Starting items: 54, 65, 75, 74".to_string(),
            "  Operation: new = old + 6".to_string(),
            "  Test: divisible by 19".to_string(),
            "    If true: throw to monkey 2".to_string(),
            "    If false: throw to monkey 0".to_string(),
            "".to_string(),
            "Monkey 2:".to_string(),
            "  Starting items: 79, 60, 97".to_string(),
            "  Operation: new = old * old".to_string(),
            "  Test: divisible by 13".to_string(),
            "    If true: throw to monkey 1".to_string(),
            "    If false: throw to monkey 3".to_string(),
            "".to_string(),
            "Monkey 3:".to_string(),
            "  Starting items: 74".to_string(),
            "  Operation: new = old + 3".to_string(),
            "  Test: divisible by 17".to_string(),
            "    If true: throw to monkey 0".to_string(),
            "    If false: throw to monkey 1".to_string(),
        ];
        assert_eq!(sum_of_two_max_monkey_inspections(input), 10605);
    }
}
// let input = vec![
//             Monkey {
//                 items: vec![83, 97, 95, 67],
//                 operation: ('*', 19),
//                 test: 17,
//                 if_true: 1,
//                 if_false: 2,
//                 inspections: 0,
//             },
//             Monkey {
//                 items: vec![71, 70, 79, 88, 56, 70],
//                 operation: ('+', 2),
//                 test: 19,
//                 if_true: 2,
//                 if_false: 0,
//                 inspections: 0,
//             },
//             Monkey {
//                 items: vec![98, 51, 51, 63, 80, 85, 84, 95],
//                 operation: ('+', 7),
//                 test: 7,
//                 if_true: 0,
//                 if_false: 1,
//                 inspections: 0,
//             }
//         ];
