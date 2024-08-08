use std::collections::{HashMap, HashSet};

use crate::starter::Solution;

impl Solution {
    /// Simple solution that validates the rules by traversing the matrix multiple times
    pub fn my_is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut hash_set = HashSet::new();
        for row in board.iter() {
            if row
                .iter()
                .filter(|c| **c != '.')
                .filter(|c| !hash_set.insert(**c))
                .count()
                > 0
            {
                return false;
            }
            hash_set.clear();
        }
        for i in 0..board.len() {
            let filter = board
                .iter()
                .map(|r| (r[i]))
                .filter(|c| *c != '.')
                .filter(|c| !hash_set.insert(*c));
            if filter.count() > 0 {
                return false;
            }
            hash_set.clear();
        }
        for i in (0..board.len()).step_by(3) {
            let cube = &board[i..i + 3];
            for j in (0..board.len()).step_by(3) {
                if cube
                    .iter()
                    .flat_map(|r| &r[j..j + 3])
                    .filter(|c| **c != '.')
                    .filter(|c| !hash_set.insert(**c))
                    .count()
                    > 0
                {
                    return false;
                }
                hash_set.clear();
            }
        }

        true
    }

    /// Improved solution that traverses the matrix only once.
    ///
    /// > Recommended approach by NeetCode
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: HashMap<usize, HashSet<char>> = HashMap::default();
        let mut cols: HashMap<usize, HashSet<char>> = HashMap::default();
        let mut squares: HashMap<(usize, usize), HashSet<char>> = HashMap::default();

        let mut row_add;
        let mut col_add;
        let mut squares_add;

        for (irow, row) in board.iter().enumerate() {
            for (jcol, col) in row.iter().enumerate() {
                if *col == '.' {
                    continue;
                }
                row_add = if let Some(val) = rows.get_mut(&irow) {
                    val.insert(*col)
                } else {
                    let mut initial = HashSet::new();
                    initial.insert(*col);
                    rows.insert(irow, initial);
                    true
                };

                col_add = if let Some(val) = cols.get_mut(&jcol) {
                    val.insert(*col)
                } else {
                    let mut initial = HashSet::new();
                    initial.insert(*col);
                    cols.insert(jcol, initial);
                    true
                };
                squares_add = if let Some(val) = squares.get_mut(&((irow / 3), (jcol / 3))) {
                    val.insert(*col)
                } else {
                    let mut initial = HashSet::new();
                    initial.insert(*col);
                    squares.insert(((irow / 3), (jcol / 3)), initial);
                    true
                };
                if !row_add || !col_add || !squares_add {
                    return false;
                }
            }
        }
        true
    }
}
