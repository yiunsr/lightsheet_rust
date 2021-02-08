const isTauri = !!window.__TAURI_INVOKE_HANDLER__;

export default {
  setTitle(title){
    window.__TAURI_INVOKE_HANDLER__({
      cmd: 'setTitle',
      title: title
    })
  },
  confirm(msg, cb){
    window.__TAURI_INVOKE_HANDLER__({
      cmd: 'confirm',
      msg: msg,
      cb: cb,
    });
  },
  prompt(msg, cb, default_input = ""){
    window.__TAURI_INVOKE_HANDLER__({
      cmd: 'prompt',
      msg: msg,
      default_input: default_input,
      cb: cb,
    });
  },
  fileOpen(path, cb){
    window.__TAURI_INVOKE_HANDLER__({
      cmd: 'fileOpen',
      path: path,
      cb: cb,
    });
  },
  fileOpenDialog(cb){
    window.__TAURI_INVOKE_HANDLER__({
      cmd: 'fileOpenDialog',
      cb: cb,
    });
  },
  initApp(){
    console.log("initApp");  
  },
  exit(){
  },
  show_progress_dialog(title){
    window.vm.$children[0].show_progress_dialog(title);
  },
  hide_progress_dialog(){
    window.vm.$children[0].hide_progress_dialog();
  },
  progress_dialog_percent(percent){
    window.vm.$children[0].progress_dialog_percent(percent);
  },
}

if(isTauri){
  window.alert = function (msg) {
    window.__TAURI_INVOKE_HANDLER__({
      cmd: 'alert',
      msg: msg
    })
  }
}
