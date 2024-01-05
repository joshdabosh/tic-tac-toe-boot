pub fn decompress_move(m: i32) -> (usize, usize) {
    let col_idx: i32 = m % 3;
    let row_idx: i32 = (m - col_idx) / 3;

    (row_idx as usize, col_idx as usize)
}