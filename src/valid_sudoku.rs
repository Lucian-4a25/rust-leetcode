// 有效的数独

use std::collections::HashMap;

struct Solution;

struct Position {
    row: usize,
    col: usize,
    region: usize,
}

const EMPTY: char = '.';

impl Position {
    fn is_conflict(&self, pos2: &Position) -> bool {
        // rule 1 & 2
        if self.row == pos2.row || self.col == pos2.col {
            return true;
        }

        // rule 3
        if self.region == pos2.region {
            return true;
        }

        false
    }
}

// 获取区域编号
#[inline]
fn get_region_number(row: usize, col: usize) -> usize {
    let row_offset = row / 3 + 1;
    let col_offset = col / 3;

    col_offset * 3 + row_offset
}

impl Solution {
    // 另外一个思路: 存储一个和原始矩阵一样长度的二维数组，数组的第一个维度表示元素的行数，
    // 第二个维度的每一个位置表示一个数字出现的次数；如果在这个数组中，第二个维度的数组
    // 如果数值大于 1，则表示当前的数独不有效。
    // 同理，可以存储一个同样的数组，验证列方向是否重复出现。
    // 对于不同的区域，可以存储一个三维数组，（当然，也可以将三维转为二维）同样的方式验证是否
    // 出现了重复数字。
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // 以粗线为分隔的区域，总共有九块，每块编号为从 1 ~ 9
        let mut state: HashMap<char, Vec<Position>> = HashMap::new();
        for (row, row_chars) in board.into_iter().enumerate() {
            for (col, ch) in row_chars.into_iter().enumerate() {
                if ch == EMPTY {
                    continue;
                }

                let pos2 = Position {
                    row,
                    col,
                    region: get_region_number(row, col),
                };
                let others = state.entry(ch).or_default();
                for other in others.iter() {
                    if other.is_conflict(&pos2) {
                        return false;
                    }
                }

                others.push(pos2);
            }
        }

        true
    }
}
