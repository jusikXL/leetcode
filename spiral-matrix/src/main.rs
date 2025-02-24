pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len(); // Number of rows
    let n = matrix[0].len(); // Number of columns

    let mut result: Vec<i32> = Vec::with_capacity(m * n);

    let (mut top, mut bottom, mut left, mut right) = (0, m - 1, 0, n - 1);

    while top <= bottom && left <= right {
        // Move right
        for j in left..=right {
            result.push(matrix[top][j]);
        }
        top += 1;

        // Move down
        if top <= bottom {
            for i in top..=bottom {
                result.push(matrix[i][right]);
            }
            right -= 1;
        }

        // Move left
        if left <= right {
            for j in (left..=right).rev() {
                result.push(matrix[bottom][j]);
            }
            bottom -= 1;
        }

        // Move up
        if top <= bottom {
            for i in (top..=bottom).rev() {
                result.push(matrix[i][left]);
            }
            left += 1;
        }
    }

    result
}

fn main() {
    println!("{:?}", spiral_order(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]))
}
