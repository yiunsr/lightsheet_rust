use tinyfiledialogs as tfd;
use tinyfiledialogs::MessageBoxIcon;
use serde_json::{Value, Map};
use webbrowser;

use web_view::{{ WebView, WVResult }};

pub fn result2js( wv: &mut WebView<usize>, success:bool, cb:String, result:String){
    if cb == "" {
        let js = &format!(
            "apiCallback({}, \"\", {});", success,  result);
        wv.eval(js);
    }
    else{
        let js = &format!(
            "apiCallback({}, {}, {});", success, cb, result);
        wv.eval(js);
    }
    
}

pub fn singleJson(key: String, value: Value) -> String{
    let mut map = Map::new();
    map.insert(key, value);
    serde_json::to_string(&map).unwrap()
}

pub fn unwrap_value_or(value:Option<&Value>, default_: String) -> String {
    match value {
        Some(x) => x.to_string(),
        None => default_,
    }
}

pub fn invoke_handler(wv: &mut WebView<usize>, arg: &str) -> WVResult {
    let parsed_json:Value = serde_json::from_str(arg).unwrap();
    let parsed_json: Map<String, Value> = parsed_json.as_object().unwrap().clone();
    let api = parsed_json["api"].as_str().unwrap().clone();
    let cb= parsed_json["cb"].as_str().unwrap().to_string().clone();
    let param = parsed_json["param"].as_object().unwrap();

    match api {
        "alert" => {
            let msg: &str = param["msg"].as_str().unwrap();
            tfd::message_box_ok("Info", msg, MessageBoxIcon::Info);
        },
        "prompt" => {
            let msg = param["msg"].as_str().unwrap();
            let title = unwrap_value_or(param.get("title"), "Light Sheet".to_string());
            let default_ = unwrap_value_or(param.get("default"), "".to_string());
            let user_input = match tfd::input_box(&title, msg, &default_){
                Some(s)=> Value::String(s),
                None => Value::Null
            };
            let result = singleJson("user_input".to_string() , user_input);
            result2js(wv, true, cb, result);
        },
        "open" => {
            let filepath = match tfd::open_file_dialog("Please choose a file...", "", None){
                Some(s) => Value::String(s),
                None => Value::Null
            };
            let result = singleJson("filepath".to_string() , filepath);
            result2js(wv, true, cb, result);
        },
        "openurl" =>{
            let url = param["url"].as_str().unwrap();
            webbrowser::open(url);
        },
        "settitle" => {
            let title = param["title"].as_str().unwrap();
            wv.set_title(title);
        }
        "save" => match tfd::save_file_dialog("Save file...", "") {
            Some(path) => tfd::message_box_ok("File chosen", &path, MessageBoxIcon::Info),
            None => tfd::message_box_ok(
                "Warning",
                "You didn't choose a file.",
                MessageBoxIcon::Warning,
            ),
        },
        "exit" => wv.exit(),
        _ => unimplemented!(),
    };
    Ok(())
}
