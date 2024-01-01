#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi::ResultExt;
use uefi::data_types::*;
use uefi::proto::console::text::*;
use core::convert::TryInto;
use log::info;


#[entry]
unsafe fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap_success();

    let mut state = [[0u8; 3]; 3];

    let mut player_x: u32 = 0;
    let mut player_y: u32 = 0;

    loop {
        display_board(state, &mut system_table, player_x, player_y);
        
        let mut events = [
            system_table.stdin()
                .wait_for_key_event()
                .unsafe_clone()
        ];

        system_table.boot_services()
            .wait_for_event(&mut events)
            .unwrap_success();

        let key = system_table.stdin().read_key().unwrap_success().unwrap();

        match key {
            Key::Printable(ch) => {
                if ch == '\r'.try_into().unwrap() {
                    info!("hit enter");
                    break;
                }
            }
            Key::Special(ch) => {
                if ch == ScanCode::RIGHT {
                    if player_x < 2 {
                        player_x += 1;
                    }
                } else if ch == ScanCode::LEFT {
                    if player_x > 0 {
                        player_x -= 1;
                    }
                } else if ch == ScanCode::UP {
                    if player_y > 0 {
                        player_y -= 1;
                    }
                } else if ch == ScanCode::DOWN {
                    if player_y < 2 {
                        player_y += 1;
                    }
                }
            }
        }

    }
    

    Status::SUCCESS
}

fn display_board(
    state: [[u8; 3]; 3],
    system_table: &mut SystemTable<Boot>,
    player_x: u32,
    player_y: u32
) -> () {
    let mut buf = [0; 1000];

    let text_board = "         |         |         \r
    1    |    2    |    3    \r
         |         |         \r
-----------------------------\r
         |         |         \r
    4    |    5    |    6    \r
         |         |         \r
-----------------------------\r
         |         |         \r
    7    |    8    |    9    \r
         |         |         \r\n";


    // since no-std, I have to do this :(
    let arr: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let arr_with_spaces: [&str; 9] = [" 1 ", " 2 ", " 3 ", " 4 ", " 5 ", " 6 ", " 7 ", " 8 ", " 9 "];
    let arr_with_parens: [&str; 9] = ["(1)", "(2)", "(3)", "(4)", "(5)", "(6)", "(7)", "(8)", "(9)"];

    // have to get around compiler complaining about
    // differntiating between &str and String, even though
    // we can't use String at all because of no-std
    let mut modified_board = text_board.replace(arr_with_spaces[(3*player_y + player_x) as usize], arr_with_parens[(3*player_y + player_x) as usize]);

    for i in 0..3 {
        for j in 0..3 {
            modified_board = modified_board.replace(arr[3*i + j],
                match state[i][j] {
                    1 => "X",
                    2 => "O",
                    _ => " "
                }
            );
        }
    }

    let s = CStr16::from_str_with_buf(&modified_board, &mut buf).expect("failed converting to buf");

    system_table.stdout().clear().unwrap_success();

    system_table.stdout()
    .output_string(&s)
    .unwrap_success();
}
