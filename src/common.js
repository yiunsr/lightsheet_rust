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
  openURL(url){
    window.__TAURI_INVOKE_HANDLER__({
      cmd: 'open',
      uri: url
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
  getTableInfo(cb){
    window.__TAURI_INVOKE_HANDLER__({
      cmd: 'getTableInfo',
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
  setStatusbar(status){
    window.vm.$children[0].statusbar = status;
  },

  getRows(from, to){
    window.__TAURI_INVOKE_HANDLER__({
      cmd: 'getRows',
      path: from,
      cb: to,
    });
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
