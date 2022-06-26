#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::{self, Window, generate_context, window};
use tauri::command;

use std::fmt::format;
use std::sync::Arc;
use std::cell::RefCell;
use std::time::Instant;

use serde_json::{self, Value};
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

#[derive(Debug, serde::Serialize)]
enum APIError {
  UnkownError,
}

fn get_window_id(window: &Window) -> u32{
  let label = window.label();
  let mut split = label.split("_");
  let _ = split.next();
  split.next().unwrap().parse::<u32>().unwrap()
}

#[command]
fn alert(msg: String){
  tfd::message_box_ok("Lightsheet", &msg, tfd::MessageBoxIcon::Info);
}

#[command]
fn confirm(msg: String) -> Result<String, APIError>{
  let result = tfd::message_box_yes_no("Lightsheet", &msg, tfd::MessageBoxIcon::Info, tfd::YesNo::Yes);
  let mut js = String::new();
  if result == tfd::YesNo::Yes{
    js.push_str("true;");
  }
  else{
    js.push_str("false;");
  }
  Ok(js)
}

#[command]
fn prompt(msg: String, default_input: String) -> Result<Value, APIError>{
  let result = match tfd::input_box("Lightsheet",&msg, &default_input){
    Some(s)=> Value::String(s),
    None => Value::Null
  };
  Ok(result)
}

#[command]
fn get_label(window: Window) -> Result<String, APIError>{
  let label = window.label();
  let res_json = format!(r#"{{"result": {}, "label": "{}"}}"#, true, label);
  Ok(res_json)
}

#[command]
async fn file_open_dialog(window: Window) -> Result<Value, APIError>{
  let filepath = match tfd::open_file_dialog("Please choose a file...", "", None){
    Some(s) => s.replace("\\", "\\\\"),
    None => String::from("")
  };
  
  let res_json = format!(r#"{{"result": {}, "filepath": "{}"}}"#, true, filepath);
  println!("{}", res_json);
  let v: Value = serde_json::from_str(&res_json).unwrap();
  Ok(v)
}

#[command]
async fn file_open(window: Window, path: String, cb: String){
  let window_id = get_window_id(&window);
  let now = Instant::now();
  println!("{}", path);
  let rfc_wv = RefCell::new(window);
  let arc_rfc_wv0 = Arc::new(rfc_wv);
  let arc_rfc_wv1 = arc_rfc_wv0.clone();
  let table_manager = get_table_manager();
  table_manager.open(1u32, path.to_string(), move |percent:u32| -> () {
    println!("{}", percent);
    let js1 = format!("common.progress_dialog_percent({})", percent);
    let aref0 = arc_rfc_wv0.borrow_mut();
    let _ = aref0.eval(&js1); 
  });
  let rowlen = table_manager.get_row_len(window_id);
  
  let js2 = format!("common.hide_progress_dialog()");
  let aref1 = arc_rfc_wv1.borrow_mut();
  let _ = aref1.eval(&js2);
  let js3 = format!("{}();", cb);
  let _ = aref1.eval(&js3);
  let spendtime = now.elapsed().as_secs_f64();
  let js4 = format!("common.log('==== spendtime : {} ====');", spendtime);
  let _ = aref1.eval(&js4);
  
  let js5 = format!("common.log('1s per insert : {}');", (rowlen as f64 / spendtime));
  let _ = aref1.eval(&js5);
}

#[command]
fn set_title(window: Window, title: String){
  let _ = window.set_title(&title);
}

#[command]
async fn get_table_info(window: Window) -> Result<Value, APIError>{
  let table_manager = get_table_manager();
  let window_id = get_window_id(&window);
  let row_len = table_manager.get_row_len(window_id);
  let col_len = table_manager.get_col_len(window_id);
  let res_json = format!(
    r#"{{"result": {}, "row_len": {}, "col_len": {}}}"#, true, row_len, col_len);
  println!("{}", res_json);
  let v: Value = serde_json::from_str(&res_json).unwrap();
  Ok(v)
}

#[command]
async fn cell_edit_done(window: Window, row_id: u32, col_index: u32, old_value: String, new_value: String) -> Result<Value, APIError>{
  let table_manager = get_table_manager();
  let window_id = get_window_id(&window);
  let _ = table_manager.cell_edit(window_id, row_id, col_index, old_value, new_value);
  let res_json = format!(r#"{{"result": {}}}"#, true);
  let v: Value = serde_json::from_str(&res_json).unwrap();
  Ok(v)
}

#[command]
async fn get_rows(window: Window, from: u32, to: u32, cb: String) -> Result<Value, APIError>{
  let table_manager = get_table_manager();
  let window_id = get_window_id(&window);
  let rows_json = table_manager.get_rows(window_id, from, to);
  let js = format!("{}({});", cb, rows_json);
  let _ = window.eval(&js);

  let res_json = format!(r#"{{"result": {}}}"#, true);
  let v: Value = serde_json::from_str(&res_json).unwrap();
  Ok(v)
}

#[command]
async fn add_rows(window: Window, row_idx: u32, row_add_count: u32, cb: String) -> Result<Value, APIError>{
  let table_manager = get_table_manager();
  let window_id = get_window_id(&window);
  table_manager.add_rows(window_id, row_idx, row_add_count);
  // let js = format!("{}({});", cb, rows_json);
  // window.eval(&js);

  let res_json = format!(r#"{{"result": {}}}"#, true);
  let v: Value = serde_json::from_str(&res_json).unwrap();
  Ok(v)
}

fn main() {
  let manager = manager::TableManager::new();
  unsafe{
    TABLE_MANAGER_WRAP = Some(manager);
  }
  let context = tauri::generate_context!("./tauri.conf.json");
  let _ = tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .invoke_handler(tauri::generate_handler![
      alert, confirm, prompt, file_open_dialog, file_open, set_title, get_label,
      get_table_info, cell_edit_done,
      get_rows, add_rows
    ]).run(context);
}
