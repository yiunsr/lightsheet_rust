#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::sync::Arc;
use std::cell::RefCell;
use std::time::Instant;

use serde_json::Value;
use tinyfiledialogs as tfd;

mod datatable;
use datatable::manager;

mod cmd;

static mut TABLE_MANAGER_WRAP:Option<manager::TableManager> = None;

fn get_table_manager<'a>() -> &'a mut manager::TableManager{
  let table_manager:& mut manager::TableManager;
  unsafe{
    table_manager = TABLE_MANAGER_WRAP.as_mut().unwrap();
  }
  table_manager
}

fn main() {
  let manager = manager::TableManager::new();
  unsafe{
    TABLE_MANAGER_WRAP = Some(manager);
  }
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
              _webview.dispatch(move |w| {
                w.eval(&js);
              });
            },
            Prompt { msg, cb, default_input} => {
              let result = match tfd::input_box("Lightsheet",&msg, &default_input){
                Some(s)=> Value::String(s),
                None => Value::Null
              };
              let js = format!("{}(`{}`);", cb, result);
              _webview.dispatch(move |w| {
                w.eval(&js);
              });
            },
            FileOpen{window_id, path, cb} =>{
              let now = Instant::now();
              println!("{}", path);
              let rfc_wv = RefCell::new(_webview);
              let arc_rfc_wv0 = Arc::new(rfc_wv);
              let arc_rfc_wv1 = arc_rfc_wv0.clone();
              let table_manager = get_table_manager();
              table_manager.open(1u32, path.to_string(), move |percent:u32| -> () {
                println!("{}", percent);
                let js1 = format!("common.progress_dialog_percent({})", percent);
                let mut aref0 = arc_rfc_wv0.borrow_mut();
                aref0.eval(&js1); 
              });
              let rowlen = table_manager.get_row_len(window_id);
              
              let js2 = format!("common.hide_progress_dialog()");
              let mut aref1 = arc_rfc_wv1.borrow_mut();
              aref1.eval(&js2);
              let js3 = format!("{}();", cb);
              aref1.eval(&js3);
              let spendtime = now.elapsed().as_secs_f64();
              let js4 = format!("common.log('==== spendtime : {} ====');", spendtime);
              aref1.eval(&js4);
              let js5 = format!("common.log('1s per insert : {}');", (rowlen as f64 / spendtime));
              aref1.eval(&js5);
            },
            FileOpenDialog{cb} =>{
              let filepath = match tfd::open_file_dialog("Please choose a file...", "", None){
                Some(s) => Value::String(s),
                None => Value::Null
              };
              println!("{}", filepath);
              let js = format!("{}({});", cb, filepath);
              _webview.dispatch(move |w| {
                w.eval(&js);
              });
            },
            SetTitle { title } => {
              _webview.dispatch(move |w| {
                w.set_title(&title);
              });
            },
            GetTableInfo {window_id, cb} =>{
              let table_manager = get_table_manager();
              let row_len = table_manager.get_row_len(window_id);
              let col_len = table_manager.get_col_len(window_id);
              let js = format!("{}({}, {});", cb, row_len, col_len);
              _webview.dispatch(move |w| {
                w.eval(&js);
              });
            },
            GetRows {window_id, from, to, cb} =>{
              let table_manager = get_table_manager();
              let rows_json = table_manager.get_rows(window_id, from, to);
              let js = format!("{}({});", cb, rows_json);
              _webview.dispatch(move |w| {
                w.eval(&js);
              });
            },
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
