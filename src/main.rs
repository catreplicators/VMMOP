// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(file_buffered)]

//#![feature(pathbuf_into_string)]

use anyhow::{Error, Result, anyhow};
//use native_dialog::{DialogBuilder, MessageLevel};
use rfd;
use slint;
use slint::*;
use std::borrow::BorrowMut;
use std::path::PathBuf;
use std::rc::Rc;
use std::result::Result::*;
use std::str::FromStr;
use std::sync::Mutex;
use std::*;

mod operation_def;
mod popup;
use operation_def::{Operation, calculate_hash};
//use popup::basic_popup;
use popup::basic_popup as basic_popup;

slint::include_modules!();

//fn setup() -> Result<(), ()> {}

//fn import_config() {}

/*fn get_dir_loc_async() -> std::path::PathBuf {
    native_dialog::DialogBuilder::file()
        .set_location("~/Desktop")
        //.add_filter("PNG Image", ["png"])
        //.add_filter("JPEG Image", ["jpg", "jpeg"])
        .add_filter("PNG Image", ["png"])
        .open_single_dir()
        .alert()
        .expect("Failed to retrieve directory.")
        .expect("Some other way to fail to retrieve a folder.")
}*/

fn get_dir_loc_sync(title: &str) -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .set_can_create_directories(true)
        .set_title(title)
        .pick_folder()
}

fn get_file_loc_sync(title: &str) -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .set_can_create_directories(true)
        .set_title(title)
        .pick_file()
}

/*fn popup_mutex_failed<T: ToString>(extra: T) -> () {
    let _ = DialogBuilder::message()
        .set_level(MessageLevel::Warning)
        .set_title("VMMOP unavailable: Other event in progress.")
        .set_text("Additional information: \n".to_string()+&extra.to_string())
        .alert();
}*/
fn popup_mutex_failed<T: ToString>(extra: T) -> () {
    basic_popup(
        "VMMOP unavailable: Other event in progress.".to_string(),
        "Additional information: \n".to_string() + &extra.to_string(),
    );
}

fn main() -> Result<(), Error> {
    //let mut rng = rand::rng();
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    /*let mut example_modifications: Vec::<(i32, SharedString)> = vec!(
        (1, SharedString::from("FirstPersonModel.jar"       )),
        (2, SharedString::from("HoldMyItems.jar"            )),
        (0, SharedString::from("Lodestone.jar"              )),
        (2, SharedString::from("Visuality.jar"              )),
        (2, SharedString::from("Extra_Nonsense.jar"         )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
        (1, SharedString::from("Filler"                     )),
    );*/
    //ui.set_modifications(ModelRc::new(VecModel::from(example_modifications)));

    let mut operation = Operation {
        name: None,
        source: PathBuf::from_str("~/Downloads/").unwrap(),
        out: PathBuf::from_str("~/Downloads/modsmodder-temp/").unwrap(),
        actions: Operation::convert_actions_manual(vec![
            (1, "FirstPersonModel.jar"),
            (2, "HoldMyItems.jar"),
            (0, "Lodestone.jar"),
            (-1, "Wonderland.exe"),
            (1, "Visuality.jar"),
            (2, "Extra Nonsense.jar"),
        ]),
    };
    ui.set_operation(operation.slintify());
    ui.set_change_colors(ModelRc::new(VecModel::from(vec![
        Color::from_argb_encoded(0xFF960000), // Error,  strong red
        Color::from_argb_encoded(0xBB5BCEFA), // Write,  pastel pink
        Color::from_argb_encoded(0xBBF5A9B8), // Read,   pastel blue
        Color::from_argb_encoded(0xBBA1BC98), // Unass., pastel green
        Color::from_argb_encoded(0xFF009600), // Unass., strong green
        Color::from_argb_encoded(0xFFC9C645), // Unass., partial yellow
    ])));

    {
        let mut a = SharedString::new();
        a.push_str("Error");
        let mut b = SharedString::new();
        b.push_str("Write");
        let mut c = SharedString::new();
        c.push_str("Delete");
        ui.set_change_labels(ModelRc::new(VecModel::from(vec![a, b, c])));
    }

    let rc_mtx_operation = Rc::new(Mutex::new(operation));

    let temp_ui_handle = ui_handle.clone();
    let mut l_mtx_op = rc_mtx_operation.clone();
    ui.on_request_get_dir_source(move || 'blk: {
        if let Ok(ref mut operation_h) = l_mtx_op.borrow_mut().try_lock() {
            let getdirres = get_dir_loc_sync("Select a source directory.");
            operation_h.source = match getdirres {
                Some(path) => path,
                None => break 'blk,
            };
            let ui = temp_ui_handle.unwrap();
            ui.set_operation(operation_h.slintify());
        } else {
            let e = anyhow!("Processing getting  a new source directory manually.");
            popup_mutex_failed(e);
        }
    });

    let temp_ui_handle = ui_handle.clone();
    let mut l_mtx_op = rc_mtx_operation.clone();
    ui.on_request_get_dir_out(move || 'blk: {
        if let Ok(ref mut operation_h) = l_mtx_op.borrow_mut().try_lock() {
            let getdirres = get_dir_loc_sync("Select an output directory.");
            operation_h.out = match getdirres {
                Some(path) => path,
                None => break 'blk,
            };
            let ui = temp_ui_handle.unwrap();
            ui.set_operation(operation_h.slintify());
        } else {
            let e = anyhow!("Processing getting  a new output directory manually.");
            popup_mutex_failed(e);
        }
    });

    let temp_ui_handle = ui_handle.clone();
    let mut l_mtx_op = rc_mtx_operation.clone();
    ui.on_request_change_details(move |details| {
        if let Ok(ref mut operation_h) = l_mtx_op.borrow_mut().try_lock() {
            let Ok(new_source) = PathBuf::from_str(details.2.as_str());
            {
                operation_h.source = new_source;
            };
            let Ok(new_out) = PathBuf::from_str(details.1.as_str());
            {
                operation_h.out = new_out;
            };
            let new_name = details.0.as_str().to_string();
            if new_name == "".to_string() {
                operation_h.name = None;
            } else {
                operation_h.name = Some(new_name);
            }
            let ui = temp_ui_handle.unwrap();
            ui.set_operation(operation_h.slintify())
        } else {
            let e = anyhow!("Processing updating manually-changed operation details.");
            popup_mutex_failed(e);
        };
    });

    let temp_ui_handle = ui_handle.clone();
    let mut l_mtx_op = rc_mtx_operation.clone();
    ui.on_request_open_operation_file(move || 'blk: {
        if let Ok(ref mut operation_h) = l_mtx_op.borrow_mut().try_lock() {
            let getfileres = get_file_loc_sync("Select an operation file.");
            let newoperation = match getfileres {
                Some(path) => match Operation::try_from_file(path) {
                    Ok(oper) => oper,
                    Err(e) => {
                        /* let _ = DialogBuilder::message()
                            .set_level(MessageLevel::Warning)
                            .set_title("Failed to open operation from file.")
                            .set_text("Additional information: \n".to_string() + &e.to_string())
                            .alert(); */
                        basic_popup(
                            "Failed to open operation from file.", 
                            "Additional information: \n".to_string() + &e.to_string()
                        );
                        break 'blk;
                    }
                },
                None => break 'blk,
            };
            **operation_h = newoperation;
            let ui = temp_ui_handle.unwrap();
            ui.set_operation(operation_h.slintify());
        } else {
            let e = anyhow!("Attempting to open an operation from a file.");
            popup_mutex_failed(e)
        }
    });

    let mut l_mtx_op = rc_mtx_operation.clone();
    ui.on_request_save_operation_file(move || 'blk: {
        if let Ok(ref mut operation_h) = l_mtx_op.borrow_mut().try_lock() {
            let save_loc = match rfd::FileDialog::new()
                .set_can_create_directories(true)
                .set_title("Save the current operation to a file.")
                .add_filter("Overwrite Automator Operation", &["vmmop", "vmmope"])
                .set_file_name(calculate_hash(&**operation_h).to_string())
                .save_file()
            {
                Some(path) => {
                    //println!("{}",path.clone().into_os_string().into_string().unwrap());
                    path
                }
                None => break 'blk,
            };
            match (**operation_h).try_save_to_file(&save_loc) {
                Ok(_) => {}
                Err(e) => {
/*                    let _ = DialogBuilder::message()
                            .set_level(MessageLevel::Warning)
                            .set_title("Failed to save an operation to a file.")
                            .set_text("Additional information: \n".to_string()+&e.to_string());
*/
                    basic_popup(
                        "Failed to save an operation to a file.",
                        "Additional information: \n".to_string() + &e.to_string(),
                    );
                    //println!("{}{}", &e, save_loc.into_os_string().into_string().unwrap());
                    break 'blk;
                }
            }
        } else {
            let e = anyhow!("Attempting to save an overwrite operation to a file.");
            popup_mutex_failed(e);
        }
    });

    let mut l_mtx_op = rc_mtx_operation.clone();
    ui.on_request_operate(move || 'blk: {
        if let Ok(ref mut operation_h) = l_mtx_op.borrow_mut().try_lock() {
            match &operation_h.exec_overwrite() {
                Ok(_r) => {}
                Err(e) => {
                    /* let _ = DialogBuilder::message()
                        .set_level(MessageLevel::Warning)
                        .set_title("Failed to complete overwrite operation.")
                        .set_text("Additional information: \n".to_string() + &e.to_string())
                        .alert(); */
                    basic_popup(
                        "Failed to complete overwrite operation.", 
                        "Additional information: \n".to_string() + &e.to_string()
                    );
                    break 'blk;
                }
            };
        } else {
            let e = anyhow!("Attempted to complete an overwrite operation.");
            popup_mutex_failed(e)
        };
    });

    ui.run()?;

    Ok(())
}
