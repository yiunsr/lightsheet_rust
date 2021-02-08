import common from './common.js'

export default {
  fileOpen(){
    common.fileOpenDialog("ui._fileOpenStart");
  },
  _fileOpenStart(filepath){
    if(filepath === null || filepath =="") return;
    common.show_progress_dialog("File Open...");
    common.fileOpen(filepath, "ui.__fileOpenEnd");
  },
  _fileOpenEnd(){
    common.hide_progress_dialog();
  },
}