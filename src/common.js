import webApi from "./webApi"

const isTauri = !!window.__TAURI_INVOKE_HANDLER__;

async function _callAPI(api, param, cb){  // eslint-disable-line no-unused-vars
  if(param == undefined){
    param = {};
  }
  if(cb == undefined){
    cb = "";
  }
  var reqParam = {api: api, param: param, cb: cb};
  var reqParamStr = JSON.stringify(reqParam);
  console.log("js api : " + api);

  if(isTauri){
    external.invoke(reqParamStr);
  }
  else{
    webApi.invoke(reqParamStr)
  }
}


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
  fileOpenDialog(cb){
    window.__TAURI_INVOKE_HANDLER__({
      cmd: 'FileOpenDialog',
      cb: cb,
    });
  },
  callAPI(api, param, cb){  // eslint-disable-line no-unused-vars
    return _callAPI(api, param, cb);
  },
  apiCallback(success, cb, result){
    if(cb == ""){
      console.log("[apiCallback] success : " + success);
    }
    else if(cb.includes(".")){
      var _1st = cb.split(".")[0];
      var _2nd = cb.split(".")[1];
      window[_1st][_2nd](success, result);
      console.log("[apiCallback] success : " + success + " cb : " + cb);
    }
    else{
      console.log("[apiCallback] success : " + success + " cb : " + cb);
      window[cb](success, result);
    }
  },
  initApp(){
    console.log("initApp");  
  },
  exit(){
    return _callAPI("exit") ;
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
