import common from './common.js'

export default {
  fileOpen(){
    common.callAPI("open", {}, "ui.retFileOpen");
  },
  retFileOpen(success, data){
    if(success === false) return;
    var filepath = data["filepath"];
    if(filepath === null || filepath =="") return;
    common.callAPI("openfile", {filepath:filepath});
    let percent = 0;
    common.show_progress_dialog();
    while(percent <= 100){
      let ret = common.callAPI("openfilePercent", {});
      percent = ret["percent"];
      common.progress_dialog_percent(percent);
    }
    common.hide_progress_dialog();
  }
}