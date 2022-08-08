import { open, save, confirm } from '@tauri-apps/api/dialog';
import common from './common.js'
import router from './router'

// debugger; // eslint-disable-line no-debugger
var _filepath;
// url.searchParams.get("path")
export default {
  fileNew(){
    let _this = this;
    let path = new URL(window.location).searchParams.get("path");
    if(path){
      confirm(window.vm.$i18n.t("menu.file_close_before_new"), { title: 'LightSheet', type: 'warning' })
        .then(function(result){
          if(result){
            _this._fileNewSaveDialog();
          }
        });
    }
    else{
      _this._fileNewSaveDialog()
    }
  },
  fileClose(){
    common.fileClose().then(function(){
      router.push({ path: '/' })
    });
  },
  _fileNewSaveDialog(){
    let path_ = '';
    save({multiple: false, filters: [{name: 'lightsheet', extensions: ['ls.db']}]})
    .then(function(path){
      if(path){
        path_ = path;
        common.fileNew(path
          ).then(function(){
            router.push({ path: 'sheet', query: { path: path_ }});
        });
      }
    });
  },
  fileImportUIDialog(){
    window.vm.$store.commit('setDialogType', "import");
  },
  fileImportNativeDialog(){
    return open({
      multiple: false,
      filters: [{
        name: 'CSV or TXT',
        extensions: ['csv', 'txt']
      }]
    })
  },
  fileImport(){
    let _this = this;
    let path = new URL(window.location).searchParams.get("path");
    if(path){
      confirm(window.vm.$i18n.t("menu.file_close_before_import"),  { title: 'LightSheet', type: 'warning' })
        .then(function(result){
          if(result){
            common.fileOpenDialog("ui._fileOpenStart").then(function(res) {
              _this._fileOpenStart(res.filepath);
            });
          }
        });
    }
    else{
      common.fileOpenDialog("ui._fileOpenStart").then(function(res) {
        _this._fileOpenStart(res.filepath);
      });
    }
  },
  fileOpen(){
    let _this = this;
    let path = new URL(window.location).searchParams.get("path");
    if(path){
      confirm(window.vm.$i18n.t("menu.file_close_before_import"),  { title: 'LightSheet', type: 'warning' })
        .then(function(result){
          if(result){
            common.fileOpenDialog("ui._fileOpenStart").then(function(res) {
              _this._fileOpenStart(res.filepath);
            });
          }
        });
    }
    else{
      common.fileOpenDialog("ui._fileOpenStart").then(function(res) {
        _this._fileOpenStart(res.filepath);
      });
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
      router.push({ path: 'sheet', query: { path: _filepath }});
      if(path)  // if already page open, reload page(because of mounted event not work)
        window.vm.$router.go()
    }, 10);
  },
  fileExport(){
    const title = window.vm.$t("menu.file_export_dialog_title");
    save({title: title, filters:
      [
        {name: 'csv', extensions: ['csv']}, {name: 'txt file', extensions: ['txt']}
      ]
    }).then(function(filepath){
      if(filepath === null)
        return;
      // debugger;  // eslint-disable-line no-debugger
      common.show_progress_dialog("File Exporing...");
      common.fileExport(filepath, "ui._fileExportEnd");
    });
  },
  _fileExportEnd(){
    setTimeout(function(){
    }, 10);
  },
}