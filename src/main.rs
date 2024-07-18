pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {

    if n == 0 {
        return Vec::new();

    }

    fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, n: usize) -> bool {
        // Check column
        for i in 0..row {
            if board[i][col] == 'Q' {
                return false;
            }
        }

        // Check 45° diagonal
        let mut i = row as i32;
        let mut j = col as i32;
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j -= 1;
        }

        // Check 135° diagonal
        let mut i = row as i32;
        let mut j = col as i32;
        while i >= 0 && j < n as i32 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j += 1;
        }

        true
    }

    fn solve(
        board: &mut Vec<Vec<char>>,
        row: usize,
        n: usize,
        solutions: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            let solution: Vec<String> = board
                .iter()
                .map(|row| row.iter().collect())
                .collect();
            solutions.push(solution);
            return;
        }

        for col in 0..n {
            if is_valid(board, row, col, n) {
                board[row][col] = 'Q';
                solve(board, row + 1, n, solutions);
                board[row][col] = '.';
            }
        }
    }

    let mut board: Vec<Vec<char>> = vec![vec!['.'; n as usize]; n as usize];
    let mut solutions: Vec<Vec<String>> = Vec::new();
    solve(&mut board, 0, n as usize, &mut solutions);
    solutions
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_n_queens_4() {
        let n = 4;
        let result = solve_n_queens(n);
        let expected = vec![
            vec![
                ".Q..".to_string(),
                "...Q".to_string(),
                "Q...".to_string(),
                "..Q.".to_string(),
            ],
            vec![
                "..Q.".to_string(),
                "Q...".to_string(),
                "...Q".to_string(),
                ".Q..".to_string(),
            ],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_n_queens_1() {
        let n = 1;
        let result = solve_n_queens(n);
        let expected = vec![vec!["Q".to_string()]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_n_queens_0() {
        let n = 0;
        let result = solve_n_queens(n);
        let expected: Vec<Vec<String>> = Vec::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_n_queens_2() {
        let n = 2;
        let result = solve_n_queens(n);
        let expected: Vec<Vec<String>> = Vec::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_n_queens_3() {
        let n = 3;
        let result = solve_n_queens(n);
        let expected: Vec<Vec<String>> = Vec::new();
        assert_eq!(result, expected);
    }
}

fn main() {
   let n = 4;

   let res = solve_n_queens(n);

   println!("{:?}" , res);

}
