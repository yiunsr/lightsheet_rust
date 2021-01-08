import webApi from "./webApi"

var WS_URI = 'ws://localhost:9010/ws/';
var _isWebview = false;
var _webSocket = null;

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

  if(_isWebview){
    external.invoke(reqParamStr);
  }
  else{
    webApi.invoke(reqParamStr)
  }
}

function init_ws(){
  function connect() {
    disconnect();
    _webSocket = new WebSocket(WS_URI);
    console.log('Connecting...');
    _webSocket.onopen = function() {
      console.log('Connected.');
    };
    _webSocket.onmessage = function(e) {
      console.log('Received: ' + e.data);
    };
    _webSocket.onclose = function() {
      console.log('Disconnected.');
      _webSocket = null;
    };
  }

  function disconnect() {
    if (_webSocket != null) {
      console.log('Disconnecting...');
      _webSocket.close();
      _webSocket = null;
    }
  }

  connect();
}

export default {
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
    if  (typeof(external) == "object" && typeof(external.invoke) != "undefined")
      _isWebview = true;
    if(_isWebview == false){
      init_ws();
    }
      
  },
  exit(){
    return _callAPI("exit") ;
  },
  show_progress_dialog(title){
    console.log("show_progress_dialog");
    window.vm_app.show_progress_dialog(title);
  },
  hide_progress_dialog(){
    console.log("hide_progress_dialog");
    window.vm_app.hide_progress_dialog();
  },
  progress_dialog_percent(percent){
    console.log("progress_dialog_percent : " + percent + "%");
    window.vm_app.progress_dialog_percent(percent);
  },
  sendWS(text){
    if(_webSocket)
      _webSocket.send(text);
  }
}
