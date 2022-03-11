use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  // your custom commands
  // multiple arguments are allowed
  // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
  MyCustomCommand { argument: String },
  Alert { msg: String },
  Confirm { msg: String, cb: String },
  Prompt { msg: String, cb: String, default_input: String },
  FileOpen { window_id: u32, path: String, cb: String },
  FileOpenDialog { cb: String },
  SetTitle { title: String },
  GetTableInfo { window_id: u32, cb: String},
  CellEditDone { window_id: u32, row_id: u32, col_index: u32, old_value: String, new_value: String},
  GetRows { window_id: u32, from: u32, to: u32, cb: String },
}
