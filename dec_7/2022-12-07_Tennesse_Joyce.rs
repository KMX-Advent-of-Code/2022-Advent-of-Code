use std::fs;

enum Line {
    // Types of lines in the input file.
    CD_ROOT,
    CD_OUT,
    CD_IN,
    LS,
    DIR,
    FILE(usize),
}

impl Line {
    fn from_string(line: &str) -> Self {
        // Parse a line from the input file with pattern matching.
        let elements: Vec<&str> = line.split_whitespace().collect();
        match elements[..] {
            ["$", "cd", "/"] => Line::CD_ROOT,
            ["$", "cd", ".."] => Line::CD_OUT,
            ["$", "cd", _] => Line::CD_IN,
            ["$", "ls"] => Line::LS,
            ["dir", _] => Line::DIR,
            [size, _] => Line::FILE(size.parse().unwrap()),
            _ => panic!("Unable to parse line {line}")
        }
    }
}

struct FilesizeAccumulator {
    // Keep track of the information we need while we're traversing
    // the filesystem.

    // Use a stack to remember information about folders we haven't
    // finished processing yet.
    stack: Vec<usize>,
    // Accumulator for the size of the folder we're currently in.
    size_counter: usize,
    // Anytime we find a file, it'll get added onto this.
    disk_space_used: usize,
    // Once we've finished processing a directory, its filesize gets
    // appended to this list.
    directory_sizes: Vec<usize>,
}

impl FilesizeAccumulator {
    fn new() -> Self {
        FilesizeAccumulator {
            stack: Vec::new(),
            size_counter: 0,
            disk_space_used: 0,
            directory_sizes: Vec::new(),
        }
    }

    fn add_file(&mut self, size: usize) {
        self.size_counter += size;
        self.disk_space_used += size;
    }
    
    fn cd_in(&mut self) {
        self.stack.push(self.size_counter);
        self.size_counter = 0;
    }
    
    fn cd_out(&mut self) {
        self.directory_sizes.push(self.size_counter);
        self.size_counter += self.stack.pop().unwrap();
    }

    fn cd_root(&mut self) {
        for _ in 0..self.stack.len() {
            self.cd_out()
        }
    }
}


fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Input file not found.");

    // Parameters from the problem statement
    let folder_size_threshold = 100_000;
    let target_disk_space = 40_000_000;

    let mut accumulator = FilesizeAccumulator::new();
    for line in input.lines().map(Line::from_string) {
        match line {
            Line::CD_ROOT => accumulator.cd_root(),
            Line::CD_OUT => accumulator.cd_out(),
            Line::CD_IN => accumulator.cd_in(),
            Line::LS => continue, // No information here.
            Line::DIR => continue, // Wait to process the directory until we cd into it.
            Line::FILE(size) => accumulator.add_file(size),
        }
    }
    // Have to cd back to the root at the end, otherwise not all directories will be counted.
    accumulator.cd_root();

    let small_folders_total_size: usize = 
        accumulator
            .directory_sizes
            .iter()
            .filter(|x| x<=&&folder_size_threshold)
            .sum()
    ;
    println!("Total size of small folders is {small_folders_total_size}");
    println!("Total disk space currently used is {}", accumulator.disk_space_used);
    let disk_space_to_clear = accumulator.disk_space_used - target_disk_space;
    println!("Need to clear {disk_space_to_clear}");
    let size_of_directory_to_delete: &usize = 
        accumulator
            .directory_sizes
            .iter()
            .filter(|x| x>= &&disk_space_to_clear)
            .min()
            .unwrap()
    ;
    println!("Size of the directory to delete is {size_of_directory_to_delete}")
}
