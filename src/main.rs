#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::prelude::*;
use uefi::ResultExt;
use uefi::data_types::*;
use uefi::proto::console::text::*;
use core::convert::TryInto;
use log::info;

#[entry]
unsafe fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap_success();

    let mut buf = [0; 170];

    let text_board = "     |     |     \r
     |     |     \r
-----------------\r
     |     |     \r
     |     |     \r
-----------------\r
     |     |     \r
     |     |     \r\n";

    let s = CStr16::from_str_with_buf(&text_board, &mut buf).expect("bruh");

    loop {
        system_table.stdout().clear().unwrap_success();

        system_table.stdout()
        .output_string(&s)
        .unwrap_success();
        
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
                    info!("bruh");
                    break;
                }
            }
            _ => {
                continue;
            }
        }

    }
    

    Status::SUCCESS
}
