import webApi from "./webApi"

var _isWebview = false;

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
  },
  exit(){
    return _callAPI("exit") ;
  },
  show_progress_dialog(title){
    window.vm_app.show_progress_dialog(title);
  },
  hide_progress_dialog(){
    window.vm_app.hide_progress_dialog();
  },
  progress_dialog_percent(percent){
    window.vm_app.progress_dialog_percent(percent);
  },
}
