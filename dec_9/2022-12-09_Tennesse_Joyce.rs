use std::fs;
use std::collections::HashSet;

fn sign(x: i32) -> i32 {
    if x > 0 {1}
    else if x < 0 {-1}
    else {0}
}

struct Knot {
    // Represents a single knot in the rope.
    x: i32,
    y: i32,
    history: HashSet<(i32, i32)>
}

impl Knot {
    fn new() -> Self {
        let mut rope = Knot {
            x: 0,
            y: 0,
            history: HashSet::new(),
        };
        rope.follow(0, 0);
        return rope;
    }
    fn follow(&mut self, lead_x: i32, lead_y: i32) {
        // Given the position of the knot we're following, move
        // to keep up.
        let offset_x = self.x - lead_x;
        let offset_y= self.y - lead_y;
        let (new_offset_x, new_offset_y) =
            match (offset_x, offset_y) {
                // If we're already adjacent to the new position, don't move.
                (x @-1..=1, y @-1..=1) => (x,y),
                // Otherwise, take a unit step in the right direction along
                // both axes (or zero if the axis already matches).
                (x, y) => (x - sign(x), y - sign(y)),
            };
        // New position is the position of the "leader" plus the new offset
        // that was computed in the previous step.
        (self.x, self.y) = (lead_x + new_offset_x, lead_y + new_offset_y);
        // Remember that we were at this new position.
        self.history.insert((self.x, self.y));
    }
}

struct RopeSimulation {
    // Represents a sequence of connected knots. Whenever we move the
    // "head" knot, the others will follow along at a delay.
    x: i32,
    y: i32,
    knots: Vec<Knot>
}

impl RopeSimulation {
    fn new(num_knots: usize) -> Self {
        let mut rope = RopeSimulation {
            x: 0,
            y: 0,
            knots: Vec::new()
        };
        for _ in 0..num_knots {
            rope.knots.push(Knot::new());
        }
        return rope;
    }

    fn move_head(&mut self, direction: &str) {
        // Move the head, and all the knots behind it.
        match direction {
            "L" => self.x -= 1,
            "R" => self.x += 1,
            "U" => self.y += 1,
            "D" => self.y -= 1,
            _ => panic!("Unknown direction: {direction}")
        }
        // Move all the knots behind it in turn.
        let (mut x, mut y) = (self.x, self.y);
        for k in self.knots.iter_mut() {
            k.follow(x, y);
            (x, y) = (k.x, k.y);
        }
    }

    fn display(&self, width: usize, height: usize, start_x: i32, start_y: i32) {
        // Print out the positions of all the knots for debugging purposes.
        // This matches the diagrams in the problem statement.
        let mut array = vec![vec!['.'; width]; height];
        array[start_y as usize][start_x as usize] = 's';
        for (i, k) in self.knots.iter().enumerate().rev() {
            let x_idx = (start_x+k.x) as usize;
            let y_idx = (start_y-k.y) as usize;
            let marker = char::from_digit((i+1) as u32, 10).unwrap();
            array[y_idx][x_idx] = marker;
        }
        array[(start_y-self.y) as usize][(start_x+self.x) as usize] = 'H';
        for row in array.iter(){
            for c in row.iter(){
                print!("{c}");
            }
            println!()
        }
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Input file not found.");
    
    let debug_mode = false;

    let mut short_rope = RopeSimulation::new(1);
    let mut long_rope = RopeSimulation::new(9);

    // Apply the instructions to both ropes.
    for line in input.lines() {
        let elements: Vec<&str> = line.split_whitespace().collect();
        let direction = elements[0];
        let num_steps = elements[1].parse().unwrap();
        for _ in 0..num_steps {
            short_rope.move_head(direction);
            long_rope.move_head(direction);
        }

        if debug_mode{
            long_rope.display(26, 21, 11, 15);
            println!();
            println!();
        }

    }

    // Part 1
    println!(
        "The tail of the short rope visits {} unique positions",
        short_rope.knots.last().unwrap().history.len()
    );
    // Part 2
    println!(
        "The tail of the long rope visits {} unique positions",
        long_rope.knots.last().unwrap().history.len()
    );
}
