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
    console.log("[apiCallback] success : " + success + " cb : " + cb);
    if(cb != "")
      window[cb](success, result);
  },
  initApp(){
    if  (typeof(external) == "object" && typeof(external.invoke) != "undefined")
      _isWebview = true;
  },
  exit(){
    return _callAPI("exit") ;
  },

}
