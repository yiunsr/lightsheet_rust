#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::env;
use std::sync::Arc;
use std::cell::RefCell;

use serde_json::Value;
use tinyfiledialogs as tfd;

mod datatable;
use datatable::manager;

mod cmd;

fn main() {
  // let key = "TAURI_CONFIG";
  // env::set_var(key, "{
  //   \"ctx\": {},
  //   \"build\":{
  //     \"devPath\": \"http://localhost:8080\"
  //   }");
  // let config_path = option_env!("TAURI_CONFIG");
  // println!("{}", config_path.unwrap());
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      println!("{}", arg);
      match serde_json::from_str(arg) {
        Err(e) => {
          Err(e.to_string())
        }
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
            },
            Alert { msg } => {
              tfd::message_box_ok("Lightsheet", &msg, tfd::MessageBoxIcon::Info);
            },
            Confirm { msg, cb } => {
              let result = tfd::message_box_yes_no("Lightsheet", &msg, tfd::MessageBoxIcon::Info, tfd::YesNo::Yes);
              let mut js = cb;
              if result == tfd::YesNo::Yes{
                js.push_str("(true);");
              }
              else{
                js.push_str("(false);");
              }
              _webview.eval(&js);
            },
            Prompt { msg, cb, default_input} => {
              let result = match tfd::input_box("Lightsheet",&msg, &default_input){
                Some(s)=> Value::String(s),
                None => Value::Null
              };
              let js = format!("{}(`{}`);", cb, result);
              _webview.eval(&js);
            },
            FileOpen{path, cb} =>{
              println!("{}", path);
              let rfc_wv = RefCell::new(_webview);
              let arc_rfc_wv0 = Arc::new(rfc_wv);
              let arc_rfc_wv1 = arc_rfc_wv0.clone();
              manager::open_file(path.to_string(), move |percent:u32| -> () {
                println!("{}", percent);
                let js1 = format!("common.progress_dialog_percent({})", percent);
                let mut aref0 = arc_rfc_wv0.borrow_mut();
                aref0.eval(&js1); 
              });
            let js2 = format!("common.hide_progress_dialog()");
            let mut aref1 = arc_rfc_wv1.borrow_mut();
            aref1.eval(&js2);
            },
            FileOpenDialog{cb} =>{
              let filepath = match tfd::open_file_dialog("Please choose a file...", "", None){
                Some(s) => Value::String(s),
                None => Value::Null
            };
            println!("{}", filepath);
            let js = format!("{}({});", cb, filepath);
             _webview.eval(&js);
            },
            SetTitle { title } => {
              _webview.dispatch(move |w| {
                w.set_title(&title);
              });
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
