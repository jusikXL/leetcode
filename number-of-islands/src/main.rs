// pub fn num_islands(grid: Vec<char>) -> i32 {
//     let mut counter = 0;

//     for (i, &x) in grid.iter().enumerate() {
//         if x == '1' {
//             // Check if the current '1' is the start of a new island
//             if i == 0 || grid[i - 1] == '0' {
//                 counter += 1;
//             }
//         }
//     }

//     counter
// }

pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let m = grid.len(); // Number of rows
    let n = grid[0].len(); // Number of columns
    let mut island_count = 0;

    // Helper function for DFS
    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let m = grid.len();
        let n = grid[0].len();

        // Base case: Out of bounds or water
        if i >= m || j >= n || grid[i][j] == '0' {
            return;
        }

        // Mark the current cell as visited (sink the land)
        grid[i][j] = '0';

        // Explore all 4 adjacent directions
        if i > 0 {
            dfs(grid, i - 1, j); // Up
        }
        if i < m - 1 {
            dfs(grid, i + 1, j); // Down
        }
        if j > 0 {
            dfs(grid, i, j - 1); // Left
        }
        if j < n - 1 {
            dfs(grid, i, j + 1); // Right
        }
    }

    // Iterate through the grid to find islands
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' {
                island_count += 1; // Found a new island
                dfs(&mut grid, i, j); // Sink the entire island
            }
        }
    }

    island_count
}

fn main() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0']
    ];
    println!("{}", num_islands(grid)); // Output: 1

    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1']
    ];
    println!("{}", num_islands(grid)); // Output: 3
}
