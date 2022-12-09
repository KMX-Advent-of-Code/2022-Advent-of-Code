use std::fs;

fn find_marker_end(signal: &Vec<char>, marker_size: usize) -> usize {
    let mut unique_count: usize = 0;
    for (i, c) in signal.iter().enumerate() {
        let mut found = false;
        for j in 1..=unique_count {
            let s = &signal[i-j];
            if s == c {
                unique_count = j;
                found = true;
                break;
            }
        }
        if !found {
            unique_count += 1;
        }
        if unique_count >= marker_size {
            return i+1;
        }
    }
    panic!("Marker not found.")
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immediate_marker() {
        let signal = vec!['a', 'b', 'c', 'd'];
        assert_eq!(find_marker_end(&signal, 4), 4);
    }

    
    #[test]
    fn delayed_marker() {
        let signal = vec!['a', 'a', 'b', 'c', 'd'];
        assert_eq!(find_marker_end(&signal, 4), 5);
    }
    
    #[test]
    fn long_delayed_marker() {
        let signal = vec!['a', 'b', 'c', 'a', 'b', 'c', 'a', 'b', 'c', 'd', 'a', 'b'];
        assert_eq!(find_marker_end(&signal, 4), 10);
    }
    
    #[test]
    fn example_from_problem_statement() {
        let signal = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect();
        assert_eq!(find_marker_end(&signal, 4), 7);
    }
    
    #[test]
    fn example_from_problem_statement_part2() {
        let signal = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect();
        assert_eq!(find_marker_end(&signal, 14), 19);
    }

}

fn main() {
    // Read the input file to a string
    let input: String = fs::read_to_string("input.txt").expect("Input file not found.");

    // Cast to a vector of chars
    let signal: Vec<char> = input.chars().collect();

    let packet_marker_end = find_marker_end(&signal, 4);
    println!("{packet_marker_end} characters need to be processed to find the start-of-packet marker.");
    
    let message_marker_end = find_marker_end(&signal, 14);
    println!("{message_marker_end} characters need to be processed to find the start-of-message marker.");
    
}
