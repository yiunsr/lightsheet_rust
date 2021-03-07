<template>
  <div>
    <div ref="mainGrid" style="width:100%;height:calc(100vh - 36px - 24px);"></div>
  </div>
</template>

<script>
  require('../../node_modules/slickgrid/slick.grid.css');
  //require('../../node_modules/slickgrid/slick-default-theme.css');
  require('../assets/plugins-gdoc-style.css');

  global.jQuery = require('jquery');
  var $ = global.jQuery;
  window.$ = $;
  require('../../node_modules/slickgrid/lib/jquery-ui-1.11.3.min.js');
  require('../../node_modules/slickgrid/lib/jquery.event.drag-2.3.0.js');
  require('../../node_modules/slickgrid/lib/jquery.event.drop-2.3.0.js');
  // debugger; // eslint-disable-line no-debugger
  require('../../node_modules/slickgrid/slick.core.js');
  require('../../node_modules/slickgrid/slick.editors.js');
  require('../../node_modules/slickgrid/slick.grid.js');
  require('../js/slick.remotemodel.js');

  window.sheet = {};
  export default {
    name: 'GridSheet',
    data: function(){
        return {
            row_len: 0, col_len: 0, colnames:[], remoteModel: {}, grid: {},
        }
    },
    mounted: function(){
      this.getTableInfo();
    },
    methods: {
      getTableInfo: function(){
        var _this = this;
        window.sheet.tableInfoCB = function(row_len, col_len){
          _this.row_len = row_len;
          _this.col_len = col_len;
          _this.colnames = _this.getColnames(col_len);
          console.log(row_len);
          console.log(col_len);
          _this.creteGrid(_this);
        }
        var common = window.common;
        common.getTableInfo('sheet.tableInfoCB');
      },
      getColname: function(i){
        return "c_" + i;
      },
      getColnames: function(col_len){
        var colnames = [];
        for(var i=0; i< col_len; i++){
          colnames.push(this.getColname(i));
        }
        return colnames;
      },
      creteGrid: function(vm){
        var util_grid = window.util_grid;
        var _columns = util_grid.getColInfos(this.col_len);
        let columns = util_grid.initColHeader(_columns);
        var options = {
          columnPicker: {
              columnTitle: "Columns"
          },
          editable: true,
          enableAddRow: true,
          enableCellNavigation: true,
          enableColumnReorder: true,
          asyncEditorLoading: false,
          autoEdit: false,
          rowHeight: 20,
        };
        var Slick = window.Slick;
        var remoteModel = new Slick.Data.RemoteModel();
        this.remoteModel = remoteModel;
        const $el = vm.$refs.mainGrid;
        var grid = new Slick.Grid($el, remoteModel.data, columns, options);
        // debugger; // eslint-disable-line no-debugger
        this.grid = grid;
        // grid.setSelectionModel(new Slick.CellSelectionModel());
        var _this = vm;
        grid.onViewportChanged.subscribe(function () {
          var vp = grid.getViewport();
          _this.remoteModel.ensureData(vp.top, vp.bottom);
        });
        grid.onSort.subscribe(function () {
          var vp = grid.getViewport();
          _this.remoteModel.ensureData(vp.top, vp.bottom);
        });
        // load the first page
        grid.onViewportChanged.notify();
      },
    },
    created : function(){
    }
  }
</script>
