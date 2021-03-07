import common from './common.js'

// debugger; // eslint-disable-line no-debugger
var _filepath;
export default {
  fileOpen(){
    common.fileOpenDialog("ui._fileOpenStart");
  },
  _fileOpenStart(filepath){
    if(filepath === null || filepath =="") return;
    _filepath = filepath;
    common.show_progress_dialog("File Open...");
    common.fileOpen(filepath, "ui._fileOpenEnd");
  },
  _fileOpenEnd(){
    setTimeout(function(){
      window.vm.$router.push({ path: 'sheet', query: { path: _filepath }});
    }, 10);
  },
}