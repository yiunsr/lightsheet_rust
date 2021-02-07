#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::env;
use tinyfiledialogs as tfd;
use serde_json::Value;

mod cmd;

fn main() {
  let key = "OUT_DIR";
  env::set_var(key, "D:\\workspace\\vscode\\lightsheet_rust\\src-tauri");
  let config_path = env!("OUT_DIR");
  println!("{}", config_path);
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
              
              let js = format!("{}(`{}`);", cb, path);
              _webview.eval(&js);  
            },
            FileOpenDialog{cb} =>{
              let filepath = match tfd::open_file_dialog("Please choose a file...", "", None){
                Some(s) => Value::String(s),
                None => Value::Null
            };
            let js = format!("{}(`{}`);", cb, filepath);
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
