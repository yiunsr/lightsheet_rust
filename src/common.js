const isTauri = !!window.__TAURI_INVOKE_HANDLER__;

export default {
  log(msg){
    console.log(msg);
  },
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
      window_id: window.window_id,
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
      window_id: window.window_id,
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
  getRows(from, to, cb){
    window.__TAURI_INVOKE_HANDLER__({
      cmd: 'getRows',
      window_id: window.window_id,
      from: from,
      to: to,
      cb: cb,
    });
  },
  toBase26(value){
    value = Math.abs(value);
    var converted = ""
         ,iteration = false
         ,remainder;
    // Repeatedly divide the numerb by 26 and convert the
    // remainder into the appropriate letter.
    do {
        remainder = value % 26;
        // Compensate for the last letter of the series being corrected on 2 or more iterations.
        if (iteration && value < 25) {
            remainder--;
        }
  
        converted = String.fromCharCode((remainder + 'A'.charCodeAt(0))) + converted;
        value = Math.floor((value - remainder) / 26);
  
        iteration = true;
    } while (value > 0);
  
    return converted;
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
