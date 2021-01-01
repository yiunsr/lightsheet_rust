import webApi from "./webApi"

var _isWebview = false;

function _callAPI(api, param){  // eslint-disable-line no-unused-vars
  if(param == undefined){
    param = "";
  }
  var reqParam = {
    api: api,
    param: param
  };
  var reqParamStr = JSON.stringify(reqParam)
  console.log("js api : " + api);
  if(_isWebview)
    return external.invoke(reqParamStr); 
  return webApi.invoke(reqParamStr)
}

export default {
  callAPI(api, param){  // eslint-disable-line no-unused-vars
    return _callAPI(api, param);
  },

  initApp(){
    if  (typeof(external) == "object" && typeof(external.invoke) != "undefined")
      _isWebview = true;
  },
  exit(){
    return _callAPI("exit") ;
  },

}