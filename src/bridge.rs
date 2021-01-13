use std::time::Duration;
use std::sync::Arc;
use std::cell::{RefCell, RefMut};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use tinyfiledialogs as tfd;
use tinyfiledialogs::MessageBoxIcon;
use serde_json::{Map, Value, to_string};
use webbrowser;
use web_view::{{ WebView, WVResult }};
use crate::datatable::manager;

pub fn result2js(success:bool, cb:String, result:String) ->String{
    let mut js_result:String;
    if cb == "" {
        let js = &format!(
            "apiCallback({}, \"\", {})", success,  result);
        js_result = js.to_string();
    }
    else{
        let js = &format!(
            "apiCallback({}, \"{}\", {})", success, cb, result);
        js_result = js.to_string();
    }
    js_result
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

pub fn invoke_handler(wv: &mut WebView<usize>,  rx:&Sender<String>, arg: &str) -> WVResult {
    let parsed_json:Value = serde_json::from_str(arg).unwrap();
    let parsed_json: Map<String, Value> = parsed_json.as_object().unwrap().clone();
    let api = parsed_json["api"].as_str().unwrap().to_string();
    let cb= parsed_json["cb"].as_str().unwrap().to_string();
    let param = parsed_json["param"].as_object().unwrap();

    if api =="settitle"{
        let title = param["title"].as_str().unwrap();
        wv.set_title(title);
        return Ok(());
    }
    else if api =="exit"{
        wv.exit();
        return Ok(());
    }

    let opt_wv = Some(wv);
    let js = invoke_handler_internal(opt_wv, rx,&api, cb, param);
    Ok(())
}

pub fn invoke_handler_internal(opt_wv: Option<&mut WebView<usize>>, rx:&Sender<String>,
        api:&str, cb: String, param: &Map<String, Value>)-> () {
    let mut js_result:String = "".to_string();
    match api {
        "simulated_api_echo"=>{
            let msg: &str = param["msg"].as_str().unwrap();
            let result = singleJson("msg".to_string() , Value::String(msg.to_string()));
            js_result = result2js(true, cb, result);
        },
        "alert" => {
            let msg: &str = param["msg"].as_str().unwrap();
            tfd::message_box_ok("Info", msg, MessageBoxIcon::Info);
            js_result = result2js(true, cb, "{}".to_string());
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
            js_result = result2js(true, cb, result);
        },
        "open" => {
            let filepath = match tfd::open_file_dialog("Please choose a file...", "", None){
                Some(s) => Value::String(s),
                None => Value::Null
            };
            let result = singleJson("filepath".to_string() , filepath);
            js_result = result2js(true, cb, result);
        },
        "openfile" => {
            let filepath = param["filepath"].as_str().unwrap();
            match opt_wv {
                Some(wv) =>{
                    let rfc_wv = RefCell::new(wv);
                    let arc_rfc_wv = Arc::new(rfc_wv);
                    {
                        let mut aref0 = arc_rfc_wv.borrow_mut();
                        let js0 = format!("common.show_progress_dialog('Loading ...')");
                        aref0.eval(&js0);
                    }
                    manager::open_file(filepath.to_string(), |percent:u32| -> () {
                        println!("{}", percent);
                        let js1 = format!("common.progress_dialog_percent({})", percent);
                        rx.send(js1).unwrap();
                        thread::sleep(Duration::from_millis(10));
                        //aref1.eval(&js1);
                    });
                    {
                        let mut aref0 = arc_rfc_wv.borrow_mut();
                        let js0 = format!("common.hide_progress_dialog()");
                        aref0.eval(&js0);
                        thread::sleep(Duration::from_millis(200));
                    }
                },
                Null => {   
                    manager::open_file(filepath.to_string(), |percent:u32| -> () {
                        println!("{}", percent);
                    });
                },
            };
            
            
            js_result = result2js(true, cb, "{}".to_string());
        },
        "openfilePercent" => {
            let percent = manager::open_file_percent() ;
            let percent = Value::from(percent);
            let result = singleJson(
                "percent".to_string() , percent);
            js_result = result2js(true, cb, result);
        },
        "openurl" =>{
            let url = param["url"].as_str().unwrap();
            webbrowser::open(url);
            js_result = result2js(true, cb, "{}".to_string());
        },
        "save" => {
            match tfd::save_file_dialog("Save file...", "") {
                Some(path) => tfd::message_box_ok("File chosen", &path, MessageBoxIcon::Info),
                None => tfd::message_box_ok(
                "Warning",
                "You didn't choose a file.",
                MessageBoxIcon::Warning,
            )};
            js_result = result2js(true, cb, "{}".to_string());
        },
        _ => unimplemented!(),
    };
    if js_result != "" {
        rx.send(js_result).unwrap();
    }
}
