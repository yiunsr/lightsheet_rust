const isTauri = !!window.__TAURI__;

export default {
  log(msg){
    console.log(msg);
  },
  setTitle(title){
    if(isTauri){
      window.__TAURI__.invoke(
        'set_title', {title: title}
      )
    }
    else{
      document.title = title;
    }
  },
  confirm(msg, cb){
    if(isTauri){
      window.__TAURI_INVOKE_HANDLER__({
        cmd: 'confirm',
        msg: msg,
        cb: cb,
      });
    }
    else{
      setTimeout(function() {
        let ret = confirm(msg);
        let js = "";
        js += cb + "(" + ret + ")";
        eval(js);
      }, 1);
    }
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
  cellEditDone(row_id, col_name, old_value, new_value){
    let col_index = col_name = this.fromBase26(col_name) - 1;
    window.__TAURI_INVOKE_HANDLER__({
      cmd: 'cellEditDone',
      window_id: window.window_id,
      row_id: row_id, col_index: col_index, old_value: old_value, new_value: new_value
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
    if(isTauri){
      window.__TAURI_INVOKE_HANDLER__({
        cmd: 'getRows',
        window_id: window.window_id,
        from: from,
        to: to,
        cb: cb,
      });
    }
    // else{
    //   setTimeout(function(){
    //     let tableData = {};
    //     cb()
    //   });
    // }
  },
  addRows(row_idx, row_add_count){
    if(isTauri){
      window.__TAURI_INVOKE_HANDLER__({
        cmd: 'addRows',
        window_id: window.window_id,
        row_idx: row_idx,
        row_add_count: row_add_count,
        cb: "",
      });
    }
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
  fromBase26(alpabet){
    // https://stackoverflow.com/a/9906193/6652082
    var base = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', i, j, result = 0;
    for (i = 0, j = alpabet.length - 1; i < alpabet.length; i += 1, j -= 1) {
      result += Math.pow(base.length, j) * (base.indexOf(alpabet[i]) + 1);
    }
  
    return result;
  },
  // https://stackoverflow.com/a/36806402/6652082
  getCSSRule(ruleName) {
    debugger; // eslint-disable-line no-debugger
    ruleName = ruleName.toLowerCase();
    var result = null;
    var find = Array.prototype.find;

    find.call(document.styleSheets, styleSheet => {
        result = find.call(styleSheet.cssRules, cssRule => {
            return cssRule instanceof CSSStyleRule 
                && cssRule.selectorText.toLowerCase() == ruleName;
        });
        return result != null;
    });
    return result;
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
