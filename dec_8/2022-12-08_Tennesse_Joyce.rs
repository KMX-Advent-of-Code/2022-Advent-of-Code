use std::fs;

fn get_digits(line: &str) -> Vec<u32> {
    // Convert a string of digits into a vector.
    line.chars().map(|x| x.to_digit(10).unwrap()).collect()
}

struct VisibilityTracker {
    height_needed: u32
}

impl VisibilityTracker {
    // Utility function that helps to check visibility of trees along
    // a line.
    fn new() -> Self {
        VisibilityTracker { height_needed: 0 }
    }

    fn next_tree(&mut self, tree_height: u32) -> bool {
        // Recieve the height of the next tree in the sequence, and
        // return a boolean indicating whether that tree is visible
        // from the outside (in this direction only).
        let is_visible = tree_height >= self.height_needed;
        if is_visible {
            self.height_needed = tree_height + 1;
        }
        return is_visible;
    }
}

struct TreePatch {
    tree_heights: Vec<Vec<u32>>
}

impl TreePatch {
    fn num_rows(&self) -> usize {
        self.tree_heights.len()
    }
    
    fn num_cols(&self) -> usize {
        self.tree_heights[0].len()
    }

    fn num_trees(&self) -> usize {
        self.num_rows() * self.num_cols()
    }

    fn get_visibility_mask(&self) -> Vec<Vec<bool>>{       
        // Get a 2d array of booleans indicating which trees are visible from
        // outside the grid.

        let num_rows = self.num_rows();
        let num_cols = self.num_cols();

        // Start by assuming all trees are hidden until we determine otherwise.
        let mut visibility_mask = vec![vec![false; num_cols]; num_rows];

        // Check left and right visibility.
        for i in 0..num_rows{
            let mut left_visibility_tracker = VisibilityTracker::new();
            let mut right_visibility_tracker = VisibilityTracker::new();
            for j in 0..num_cols{
                // Check visibility from the left.
                visibility_mask[i][j] |= left_visibility_tracker.next_tree(
                    self.tree_heights[i][j]);
                // Check visibility from the right.
                let jp = num_cols - j - 1;
                visibility_mask[i][jp] |= right_visibility_tracker.next_tree(
                    self.tree_heights[i][jp]);
            }
        }

        // Check up and down visibility.
        for j in 0..num_cols{
            let mut up_visibility_tracker = VisibilityTracker::new();
            let mut down_visibility_tracker = VisibilityTracker::new();
            for i in 0..num_rows{
                // Check visibility from up.
                visibility_mask[i][j] |= up_visibility_tracker.next_tree(
                    self.tree_heights[i][j]);
                // Check visibility from down.
                let ip = num_rows - i - 1;
                visibility_mask[ip][j] |= down_visibility_tracker.next_tree(
                    self.tree_heights[ip][j]);
            }
        }

        return visibility_mask
    }

    fn get_scenic_score(&self, i: usize, j: usize) -> i32 {
        // Calculate the scenic score of the tree at position i, j.
        let mut viewing_distances = [0; 4];
        let tree_height = self.tree_heights[i][j];
        // Check down.
        for ip in i+1..self.num_rows(){
            viewing_distances[0] += 1;
            if self.tree_heights[ip][j] >= tree_height {
                break;
            }
        }
        // Check up.
        for ip in (0..i).rev(){
            viewing_distances[1] += 1;
            if self.tree_heights[ip][j] >= tree_height {
                break;
            }
        }
        // Check right.
        for jp in j+1..self.num_cols(){
            viewing_distances[2] += 1;
            if self.tree_heights[i][jp] >= tree_height {
                break;
            }
        }
        // Check left.
        for jp in (0..j).rev(){
            viewing_distances[3] += 1;
            if self.tree_heights[i][jp] >= tree_height {
                break;
            }
        }
        // Scenic score is defined as the product of the viewing distances.
        viewing_distances.iter().product()        
    }

}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Input file not found.");
    let tree_patch = TreePatch{
        tree_heights: input.lines().map(get_digits).collect()
    };

    // Part 1
    let visibility_mask = tree_patch.get_visibility_mask();
    let num_visible_trees =
        visibility_mask
            .iter()
            .flatten()
            .filter(|x| **x)
            .count()
    ;    
    println!("{} out of {} trees are visible.", num_visible_trees, tree_patch.num_trees());

    // Part 2
    let mut scenic_scores: Vec<i32> = Vec::new();
    for i in 0..tree_patch.num_rows(){
        for j in 0..tree_patch.num_cols(){
            scenic_scores.push(tree_patch.get_scenic_score(i, j));
        }
    }
    let max_scenic_score = scenic_scores.iter().max().unwrap();
    println!("Max scenic score is {max_scenic_score}.");

}
