// for each of these 8 coordinates, check if each is true or false
// for cell, put in these coordinates, then run

// x = A-1, y = B-1
// x = A-1, y = B
// x = A-1, y = B+1
// x = A, y = B-1
// x = A, y = B+1
// x = A +1, y = B-1
// x =A+1, y=B
// x = A+1, y= B+1

claude's version
fn check_neighbor_cells(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> Vec<(usize, usize, bool)> {
let rows = grid.len();
let cols = grid[0].len();

    // Define the 8 neighboring cell coordinates relative to the input cell
    let neighbor_offsets = [
        (-1, -1), // x = A-1, y = B-1
        (-1, 0),  // x = A-1, y = B
        (-1, 1),  // x = A-1, y = B+1
        (0, -1),  // x = A, y = B-1
        (0, 1),   // x = A, y = B+1
        (1, -1),  // x = A+1, y = B-1
        (1, 0),   // x = A+1, y = B
        (1, 1)    // x = A+1, y = B+1
    ];

    // Vector to store neighbor cell information
    let mut neighbor_cells = Vec::new();

    // Check each neighboring cell
    for (dx, dy) in neighbor_offsets.iter() {
        // Calculate new coordinates
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        // Check if the new coordinates are within grid bounds
        if new_x >= 0 && new_x < rows as i32 &&
            new_y >= 0 && new_y < cols as i32 {
            // Get the cell value and store its coordinates and status
            let cell_value = grid[new_x as usize][new_y as usize];
            neighbor_cells.push((new_x as usize, new_y as usize, cell_value));
        }
    }

    neighbor_cells
}

fn main() {
// Example grid
let grid = vec![
vec![false, true, false],
vec![true, false, true],
vec![false, true, false]
];

    // Example usage
    let neighbors = check_neighbor_cells(&grid, 1, 1);

    // Print out the neighboring cells
    println!("Neighboring cells:");
    for (x, y, value) in neighbors {
        println!("Coordinates ({}. {}): {}", x, y, value);
    }
}