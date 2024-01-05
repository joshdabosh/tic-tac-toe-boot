use uefi::prelude::Boot;
use uefi::prelude::SystemTable;

use uefi::CStr16;

#[allow(dead_code)]
pub fn decompress_move(m: i32) -> (usize, usize) {
    let col_idx: i32 = m % 3;
    let row_idx: i32 = (m - col_idx) / 3;

    (row_idx as usize, col_idx as usize)
}

#[allow(dead_code)]
pub fn print_string_literal(
    system_table: &mut SystemTable<Boot>,
    string: &str
) -> () {
    let mut buf = [0; 1000];

    let s = CStr16::from_str_with_buf(&string, &mut buf)
        .unwrap();

    system_table.stdout()
        .clear()
        .unwrap();

    system_table.stdout()
        .output_string(&s)
        .unwrap();
}

#[allow(dead_code)]
pub fn pause_execution(
    system_table: &mut SystemTable<Boot>,
    time: usize
) -> () {
    system_table
        .boot_services()
        .stall(time);
}