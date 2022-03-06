import common from './common.js'

// debugger; // eslint-disable-line no-debugger
var _filepath;
// url.searchParams.get("path")
export default {
  fileOpen(){
    let path = new URL(window.location).searchParams.get("path");
    if(path){
      window.__menu__fileOpen = function(result){
        if(result)
          common.fileOpenDialog("ui._fileOpenStart");
      };
      common.confirm(window.vm.$i18n.t("menu.file_close_before_open"), "__menu__fileOpen");
    }
    else{
      common.fileOpenDialog("ui._fileOpenStart");
    }
  },
  _fileOpenStart(filepath){
    if(filepath === null || filepath =="") return;
    _filepath = filepath;
    common.show_progress_dialog("File Open...");
    common.fileOpen(filepath, "ui._fileOpenEnd");
  },
  _fileOpenEnd(){
    setTimeout(function(){
      let path = new URL(window.location).searchParams.get("path");
      window.vm.$router.push({ path: 'sheet', query: { path: _filepath }});
      if(path)  // if already page open, reload page(because of mounted event not work)
        window.vm.$router.go()
    }, 10);
  },
}