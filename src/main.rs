#![no_main]
#![no_std]

extern crate alloc;

use uefi::prelude::*;
use uefi::ResultExt;
use uefi::data_types::*;
use uefi::proto::console::text::*;
use uefi::table::runtime::*;
use uefi::table::boot::*;
use log::info;

use uefi::proto::media::fs::SimpleFileSystem;

use uefi::proto::device_path::DevicePath;
use uefi::proto::device_path::text::DevicePathToText;
use uefi::proto::device_path::text::DisplayOnly;
use uefi::proto::device_path::text::AllowShortcuts;

use uefi::proto::media::file::File;
use uefi::proto::media::file::RegularFile;
use uefi::proto::media::file::FileMode;
use uefi::proto::media::file::FileAttribute;
use uefi::proto::media::file::FileInfo;

use alloc::string::ToString;
use alloc::format;
use alloc::vec;

#[entry]
unsafe fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    let mut state = [[0u8; 3]; 3];

    let mut player_x: u32 = 0;
    let mut player_y: u32 = 0;

    let mut ctr = 0;

    loop {
        display_board(state, &mut system_table, player_x, player_y);
        
        let mut events = [
            system_table.stdin()
                .wait_for_key_event()
                .unwrap()
        ];

        system_table.boot_services()
            .wait_for_event(&mut events)
            .unwrap();

        let key = system_table.stdin().read_key().unwrap().unwrap();

        match key {
            Key::Printable(ch) => {
                if ch == Char16::from_u16_unchecked(0xd) {
                    if ctr > 9 {
                        handle_endgame(&mut system_table);
                    }

                    ctr += 1;

                    info!("hit enter");
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

    system_table.stdout().clear().unwrap();

    system_table.stdout()
        .output_string(&s)
        .unwrap();
}

fn handle_endgame(system_table: &mut SystemTable<Boot>) -> () {
    proceed_with_boot(system_table);
}

fn proceed_with_boot(system_table: &mut SystemTable<Boot>) -> () {
    let device_path_to_text_handle = system_table
        .boot_services()
        .get_handle_for_protocol::<DevicePathToText>()
        .unwrap();

    let device_path_to_text = system_table
        .boot_services()
        .open_protocol_exclusive::<DevicePathToText>(device_path_to_text_handle)
        .unwrap();


    let handle_buffers = system_table
        .boot_services()
        .locate_handle_buffer(SearchType::ByProtocol(&SimpleFileSystem::GUID))
        .unwrap();
    
    for handle in handle_buffers.iter() {
        let device_path = system_table
            .boot_services()
            .open_protocol_exclusive::<DevicePath>(*handle);

        if device_path.is_err() {
            continue;
        }

        let device_path = device_path.unwrap();

        let mut found = false;

        for node in device_path.node_iter() {
            let text = device_path_to_text
                .convert_device_node_to_text(
                    system_table.boot_services(),
                    node,
                    DisplayOnly(false),
                    AllowShortcuts(true)
                )
                .unwrap();

            if text.to_string().to_lowercase() == "ata(primary,master,0x0)" {
                found = true;
            }
        }

        if !found {
            continue;
        }

        for node in device_path.node_iter() {
            let node_ptr = node.as_ffi_ptr();
            let mut dev_ptr = unsafe { DevicePath::from_ffi_ptr(node_ptr) };

            let can_load_fs = system_table
                .boot_services()
                .locate_device_path::<SimpleFileSystem>(&mut dev_ptr);

            if can_load_fs.is_err() {
                info!("erroring out");
                continue;
            }

            let mut loaded = system_table
                .boot_services()
                .open_protocol_exclusive::<SimpleFileSystem>(can_load_fs.unwrap());

            if loaded.is_err() {
                continue;
            }

            let mut volume = loaded.unwrap().open_volume().unwrap();

            let mut fname_buf = [0; 100];

            let fname = CStr16::from_str_with_buf("EFI\\vmlinuz-linux", &mut fname_buf).unwrap();

            let fhandle = volume.open(fname, FileMode::Read, FileAttribute::all())
                .map_err(|e| format!("ERROR: {e:?}")).unwrap();

            let mut fcontent = fhandle.into_regular_file().unwrap();

            let mut finfo_buf = [0; 1000];

            let finfo = fcontent.get_info::<FileInfo>(&mut finfo_buf).unwrap();

            let file_size: usize = finfo.file_size().try_into().unwrap();

            let mut fcontent_buf = vec![0; file_size];

            let read_bytes = fcontent.read(&mut fcontent_buf).unwrap();

            if read_bytes != file_size {
                panic!("read wrong amount of bytes from kernel");
            }

        }

        info!("OK SO FAR");
    }

    system_table.boot_services().stall(5000000);
}