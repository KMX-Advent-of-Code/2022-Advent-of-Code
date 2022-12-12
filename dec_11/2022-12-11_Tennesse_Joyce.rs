use std::collections::VecDeque;

impl Monkey {
    fn inspect_item(&self, worry: isize) -> (usize, isize) {
        let worry = adjust_worry(worry, &self.operation);
        // Uncomment this for Part 1
        // let worry = worry / 3;
        if (worry % self.divisible_by) == 0 {
            return (self.recipients.0, worry);
        }
        else {
            return (self.recipients.1, worry);
        }
    }

    fn take_turn(&mut self) -> VecDeque<(usize, isize)> {
        let mut items_thrown = VecDeque::new();
        while self.items.len() > 0 {
            self.total_items_inspected += 1;
            let worry = self.items.pop_front().unwrap();
            let action = self.inspect_item(worry);
            items_thrown.push_back(action);
        }
        return items_thrown;
    }
}

struct Monkey {
    items: VecDeque<isize>,
    operation: Operation,
    divisible_by: isize,
    recipients: (usize, usize),
    total_items_inspected: isize
}

enum Operation {
    // Categorizes the types of operations in the input file.
    Add(isize),
    Multiply(isize),
    Square
}

fn adjust_worry(worry: isize, operation: &Operation) -> isize {
    match operation {
        Operation::Add(x) => worry + x,
        Operation::Multiply(x) => worry * x,
        Operation::Square => worry * worry,
    }
}

fn test_configure_monkeys() -> Vec<Monkey> {
    // This sets up the test configuration of monkeys used in the problem
    // statement. I just copied the info over manually. Since there are only
    // a handful of monkeys that seemed faster than writing a parser.
    vec![
        Monkey { // 0
            items: VecDeque::from([79, 98]),
            operation: Operation::Multiply(19),
            divisible_by: 23,
            recipients: (2,3),
            total_items_inspected: 0
        },
        Monkey { // 1
            items: VecDeque::from([54, 65, 75, 74]),
            operation: Operation::Add(6),
            divisible_by: 19,
            recipients: (2,0),
            total_items_inspected: 0
        },
        Monkey { // 2
            items: VecDeque::from([79, 60, 97]),
            operation: Operation::Square,
            divisible_by: 13,
            recipients: (1,3),
            total_items_inspected: 0
        },
        Monkey { // 3
            items: VecDeque::from([74]),
            operation: Operation::Add(3),
            divisible_by: 17,
            recipients: (0,1),
            total_items_inspected: 0
        },
    ]
}

fn configure_monkeys() -> Vec<Monkey> {
    // This sets up the configuration of monkeys in my input file.
    // I just copied the info over manually. Since there are only
    // a handful of monkeys that seemed faster than writing a parser.
    vec![
        Monkey { // 0
            items: VecDeque::from([54, 61, 97, 63, 74]),
            operation: Operation::Multiply(7),
            divisible_by: 17,
            recipients: (5,3),
            total_items_inspected: 0
        },
        Monkey { // 1
            items: VecDeque::from([61, 70, 97, 64, 99, 83, 52, 87]),
            operation: Operation::Add(8),
            divisible_by: 2,
            recipients: (7,6),
            total_items_inspected: 0
        },
        Monkey { // 2
            items: VecDeque::from([60, 67, 80, 65]),
            operation: Operation::Multiply(13),
            divisible_by: 5,
            recipients: (1,6),
            total_items_inspected: 0
        },
        Monkey { // 3
            items: VecDeque::from([61, 70, 76, 69, 82, 56]),
            operation: Operation::Add(7),
            divisible_by: 3,
            recipients: (5,2),
            total_items_inspected: 0
        },
        Monkey { // 4
            items: VecDeque::from([79, 98]),
            operation: Operation::Add(2),
            divisible_by: 7,
            recipients: (0,3),
            total_items_inspected: 0
        },
        Monkey { // 5
            items: VecDeque::from([72, 79, 55]),
            operation: Operation::Add(1),
            divisible_by: 13,
            recipients: (2,1),
            total_items_inspected: 0
        },
        Monkey { // 6
            items: VecDeque::from([63]),
            operation: Operation::Add(4),
            divisible_by: 19,
            recipients: (7,4),
            total_items_inspected: 0
        },
        Monkey { // 7
            items: VecDeque::from([72, 51, 93, 63, 80, 86, 81]),
            operation: Operation::Square,
            divisible_by: 11,
            recipients: (0,4),
            total_items_inspected: 0
        }
    ]
}



fn main() {
    let mut monkeys = configure_monkeys();
    // Use 20 rounds for Part 1.
    let num_rounds = 10_000;

    // I'm using "modular arithmetic" to keep the worry levels from getting
    // too large. That means whenever the worry level grows above some maximum
    // value, it loops back around to zero. In order to keep the number of items
    // exchanged the same as in the "true" calculation, the max worry level should
    // be divisible by every monkey's "divisible_by" number, so I'm just using
    // the product. The least common multiple would be more economical, but this
    // is simpler and, at least for the input data I was given, the two are the same.
    let max_worry_level: isize = monkeys.iter().map(|x| x.divisible_by).product();
    println!("Max worry level is {max_worry_level}.");

    let num_monkeys = monkeys.len();
    for turn_idx in 0..(num_monkeys * num_rounds) {
        let monkey_idx = turn_idx % num_monkeys;
        let monkey = &mut monkeys[monkey_idx];
        let items_thrown = monkey.take_turn();
        for (recipient, item) in items_thrown.iter(){
            // Comment out the next line for Part 1.
            let worry_level = item % max_worry_level;
            monkeys[*recipient].items.push_back(worry_level);
        }

        // Print out what each monkey is holding when the round ends, for debugging.
        // if (turn_idx+1) % num_monkeys == 0 {
        //     println!("After round {}, the monkeys are holding items with these worry levels:", (turn_idx+1) / num_monkeys);
        //     for (i, monkey) in monkeys.iter().enumerate() {
        //         print!("Monkey {i}: ");
        //         for item in monkey.items.iter(){
        //             print!("{item}, ");
        //         }
        //         println!();
        //     }
        // }
    }
    
    let mut items_inspected: Vec<isize> = monkeys.iter().map(|x| x.total_items_inspected).collect();
    for (i, items_inspected) in items_inspected.iter().enumerate() {
        println!("Monkey {} inspected items {} times.", i, items_inspected);
    }
    items_inspected.sort();
    let monkey_business = items_inspected[num_monkeys-1] * items_inspected[num_monkeys-2];
    println!("The level of monkey business in this situation is {monkey_business}.");


}