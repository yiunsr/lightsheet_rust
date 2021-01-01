use tinyfiledialogs as tfd;
use tinyfiledialogs::MessageBoxIcon;
use serde_json::{Value, Map};
use webbrowser;

use web_view::{{ WebView, WVResult }};

pub fn invoke_handler(wv: &mut WebView<usize>, arg: &str) -> WVResult {
    let parsed_json:Value = serde_json::from_str(arg).unwrap();
    let api = parsed_json["api"].as_str().unwrap();

    match api {
        "open" => match tfd::open_file_dialog("Please choose a file...", "", None) {
            Some(path) => tfd::message_box_ok("File chosen", &path, MessageBoxIcon::Info),
            None => tfd::message_box_ok(
                "Warning",
                "You didn't choose a file.",
                MessageBoxIcon::Warning,
            ),
        },
        "openurl" =>{
            let url = parsed_json["param"].as_str().unwrap();
            webbrowser::open(url);
        },
        "save" => match tfd::save_file_dialog("Save file...", "") {
            Some(path) => tfd::message_box_ok("File chosen", &path, MessageBoxIcon::Info),
            None => tfd::message_box_ok(
                "Warning",
                "You didn't choose a file.",
                MessageBoxIcon::Warning,
            ),
        },
        "alert" => {
            let param = parsed_json["param"].as_str().unwrap();
            tfd::message_box_ok("Info", param, MessageBoxIcon::Info)
        }
        "info" => {
            tfd::message_box_ok("Info", "This is a info dialog", MessageBoxIcon::Info)
        }
        "warning" => tfd::message_box_ok(
            "Warning",
            "This is a warning dialog",
            MessageBoxIcon::Warning,
        ),
        "error" => {
            tfd::message_box_ok("Error", "This is a error dialog", MessageBoxIcon::Error)
        }
        "exit" => wv.exit(),
        _ => unimplemented!(),
    };
    Ok(())
}
