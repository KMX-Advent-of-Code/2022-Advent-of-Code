use std::fs;

struct ClockCircuit {
    current_value: i32,
    history: Vec<i32>
}

impl ClockCircuit {
    fn new() -> Self {
        ClockCircuit {
            current_value: 1,
            history: vec![1]
        }
    }

    fn execute(&mut self, instruction: &str) {
        //Execute an instruction from the input file.
        
        // Both types of instructions do this step.
        self.history.push(self.current_value);
        
        let elements: Vec<&str> = instruction.split_whitespace().collect();
        match elements[..] {
            ["noop"] => {},
            ["addx", x] => {
                self.current_value += x.parse::<i32>().unwrap();
                self.history.push(self.current_value);
            },
            _ => panic!("Could not parse instruction {instruction}")
        }
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Input file not found.");
    
    let mut clock_circuit = ClockCircuit::new();
    for line in input.lines(){
        clock_circuit.execute(line);
    }

    // Part 1
    // Add up the signal strength at cycles 20, 60, 100, ...
    let mut total_signal_strength = 0;
    for i in (20..=220).step_by(40){
        let value = clock_circuit.history[i-1];
        let signal_strength = value * (i as i32);
        println!("Signal strength at cycle {i} is {i} * {value} = {signal_strength}.");
        // println!("Nearby values are {}, {}, {}, {}, {}", clock_circuit.history[i-3], clock_circuit.history[i-2], clock_circuit.history[i-1], clock_circuit.history[i], clock_circuit.history[i+1]);
        total_signal_strength += signal_strength;
    }
    println!("Total signal strength is {total_signal_strength}");

    // Part 2
    // Display the output on the CRT screen.
    for (i, value) in clock_circuit.history.iter().enumerate(){
        let crt_x = (i % 40) as i32;
        if crt_x == 0 {
            println!()
        }
        if (value + 1 >= crt_x) & (value - 1 <= crt_x){
            print!("#");
        }
        else {
            print!(".");
        }
    }


}
