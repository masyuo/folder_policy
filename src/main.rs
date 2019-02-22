extern crate winreg;
extern crate iui;

use winreg::RegKey;
use winreg::enums::*;
use std::io;
use iui::prelude::*;
use iui::controls::{Checkbox, Group, LayoutGrid, GridAlignment, GridExpand};

fn key_value(val : bool) -> String {
    let mut ret = String::new();
    if val == true {
        ret.push_str("Hide");
    } else {
        ret.push_str("Show");
    }

    return ret;
}

fn toggle_key(path : &[&str], value : &mut [bool], index: usize) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.open_subkey_with_flags(path[index], KEY_ALL_ACCESS).expect("Failed to open subkey");
    key.set_value("ThisPCPolicy", &key_value(value[index])).expect("Failed to write value");
    value[index] = !value[index]
}

fn toggle_desktop(value : &mut [bool], index: usize){
	let path: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FolderDescriptions\{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}\PropertyBag";
	let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
	let key = hklm.open_subkey_with_flags(path, KEY_ALL_ACCESS).expect("Failed to open subkey");
	if value[index] == true {
		key.create_subkey("ThisPCPolicy").unwrap();
		key.set_value("ThisPCPolicy", &String::from("Hide")).expect("Failed to write value");
	} else {
		key.delete_value("ThisPCPolicy").unwrap();
	}
	value[index] = !value[index]
}

fn main() {
    let paths : [&str; 5] = [
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FolderDescriptions\{f42ee2d3-909f-4907-8871-4c22fc0bf756}\PropertyBag",       //Documents
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FolderDescriptions\{0ddd015d-b06c-45d5-8c4c-f59713854639}\PropertyBag",       //Pictures
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FolderDescriptions\{35286a68-3c57-41a1-bbb1-0eae73d76c95}\PropertyBag",       //Videos
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FolderDescriptions\{7d83ee9b-2244-4e70-b1f5-5393042af1e4}\PropertyBag",       //Downloads
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FolderDescriptions\{a0c69a99-21c8-4671-8703-7934162fcf1d}\PropertyBag"        //Music
    ];

    let mut enabled : [bool; 6] = Default::default();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    for i in 0..paths.len() {
        let key = hklm.open_subkey(paths[i]).expect("Failed to open subkey");
        let value : String = key.get_value("ThisPCPolicy").expect("Failed to find value");

        if value == "Show" {
            enabled[i] = true;
        } else {
            enabled[i] = false;
        }
    }

	let path: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FolderDescriptions\{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}\PropertyBag";
	let key = hklm.open_subkey(path).expect("Failed to open subkey");
	let value : String = key.get_value("ThisPCPolicy").unwrap_or_else(|e| match e.kind() {
		io::ErrorKind::NotFound => String::new(),
		_ => panic!("{:?}", e)
	});
	if value == "" {
		enabled[5] = true
	} else {
		enabled[5] = false
	}

    let ui = UI::init().expect("Couldn't initialize ui");
    let mut win = Window::new(&ui, "Test App", 200, 200, WindowType::NoMenubar);

    let mut group = Group::new(&ui, "Folders");
    let mut layout = LayoutGrid::new(&ui);
    layout.set_padded(&ui, true);

    let mut documents_toggle = Checkbox::new(&ui, "Documents");
    documents_toggle.set_checked(&ui, enabled[0]); 

    let mut pictures_toggle = Checkbox::new(&ui, "Pictures");
    pictures_toggle.set_checked(&ui, enabled[1]);

    let mut videos_toggle = Checkbox::new(&ui, "Videos");
    videos_toggle.set_checked(&ui, enabled[2]);

    let mut downloads_toggle = Checkbox::new(&ui, "Downloads");
    downloads_toggle.set_checked(&ui, enabled[3]);

    let mut music_toggle = Checkbox::new(&ui, "Music");
    music_toggle.set_checked(&ui, enabled[4]);

	let mut desktop_toggle = Checkbox::new(&ui, "Desktop");
	desktop_toggle.set_checked(&ui, enabled[5]);

    //This is so gross.
    documents_toggle.on_toggled(&ui, {
        let _ui = ui.clone();
        move |_val| {
            toggle_key(&paths, &mut enabled, 0);
        }
    });
    pictures_toggle.on_toggled(&ui, {
        let _ui = ui.clone();
        move |_val| {
            toggle_key(&paths, &mut enabled, 1);
        }
    });
    videos_toggle.on_toggled(&ui, {
        let _ui = ui.clone();
        move |_val| {
            toggle_key(&paths, &mut enabled, 2);
        }
    });
    downloads_toggle.on_toggled(&ui, {
        let _ui = ui.clone();
        move |_val| {
            toggle_key(&paths, &mut enabled, 3);
        }
    });
    music_toggle.on_toggled(&ui, {
        let _ui = ui.clone();
        move |_val| {
            toggle_key(&paths, &mut enabled, 4);
        }
    });
	desktop_toggle.on_toggled(&ui, {
		let _ui = ui.clone();
		move |_val| {
			toggle_desktop(&mut enabled, 5)
		}
	});

    layout.append(&ui, documents_toggle, 0, 0, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
    layout.append(&ui, pictures_toggle, 0, 1, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
    layout.append(&ui, videos_toggle, 0, 2, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
    layout.append(&ui, downloads_toggle, 1, 0, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
    layout.append(&ui, music_toggle, 1, 1, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
	layout.append(&ui, desktop_toggle, 1, 2, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);

    group.set_child(&ui, layout);

    win.set_child(&ui, group);
    win.show(&ui);
    ui.main();
}