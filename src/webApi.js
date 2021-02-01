function invokeInternal(reqParamStr){
  var reqParam = JSON.parse(reqParamStr);
  var api = reqParam["api"];
  var param = reqParam["param"];
  var cb = reqParam["cb"];

  var ret = doAction(api, param, reqParamStr);
  if(ret === null) return;
  var success = ret[0];
  var result = ret[1];
  window.apiCallback (success, cb, result);
}

function doAction(api, param, reqParamStr){
  var filepath;
  console.log(reqParamStr);
  switch(api){
    case "alert":
      alert(param["msg"]);
      break;
    case "prompt":
      var user_input = prompt(param["msg"]);
      return [true, {user_input: user_input}];
    case "open":
      filepath = window.prompt(
        "simulated file\nInput File path",
        "D:\\res\\csv_sample\\세종1.txt");
      return [true, {filepath: filepath}];
    case "openurl":
      var url = param["url"];
      window.open(url, '_blank');  
      break;
    case "settitle":
      document.title = param["title"];
      break;
    case "exit":
      self.opener = self;
      window.close();
      alert("Exit API called");
      break;
    default:
      console.log("webApi : api Not Implement");
      return [false, {}];
  }
  return [true, {}];
}

export default {
  invoke(reqParamStr){
    setTimeout(function() {
      invokeInternal(reqParamStr);
  }, 1)
    return {success: true}
  },
}
