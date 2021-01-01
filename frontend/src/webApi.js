

export default {
  invoke(reqParamStr){
    var reqParam = JSON.parse(reqParamStr);
    var api = reqParam["api"];
    var param = reqParam["param"];
    switch(api){
      case "alert":
        alert(param);
        break;
      case "open":
        var input = document.createElement('input');
        input.type = 'file';
        input.click();
        break;
      case "openurl":
        var url = param;
        window.open(url, '_blank');  
        break;
      case "exit":
        self.opener = self;
        window.close();
        alert("Exit API called");
        break;
      default:
        console.log("webApi : api Not Implement");
        break;
    }
  },
}
