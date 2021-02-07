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
  FileOpen { path: String, cb: String },
  FileOpenDialog { cb: String },
  SetTitle { title: String },
}
