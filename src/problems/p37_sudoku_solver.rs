use super::Solution;

impl Solution {
    const RANGE: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }

    fn solve(board: &mut [Vec<char>]) {
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    for k in Self::RANGE {
                        if Self::check(board, i, j, k) {
                            board[i][j] = k;
                            Self::solve(board);

                            if !board.iter().any(|row| row.iter().any(|&val| val == '.')) {
                                return;
                            }

                            board[i][j] = '.';
                        }
                    }
                    return;
                }
            }
        }
    }

    fn check(board: &mut [Vec<char>], row: usize, col: usize, num: char) -> bool {
        if board[row].iter().any(|&val| val == num) {
            return false;
        }
        if board.iter().any(|r| r[col] == num) {
            return false;
        }

        let rs = (row / 3) * 3;
        let cs = (col / 3) * 3;

        for r in rs..rs + 3 {
            for c in cs..cs + 3 {
                if board[r][c] == num {
                    return false;
                }
            }
        }

        true
    }
}
