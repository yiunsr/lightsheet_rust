<style>
.slick-cell.grid-row-hader{
  background: #f0f0f0;
}
.slick-row .slick-cell.active{
  padding-top: 2px;
}
</style>

<template>
  <div  ref="viewport">
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
  require('jquery-ui/ui/widgets/sortable.js');
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
      // debugger; // eslint-disable-line no-debugger
      this.init();
    },
    beforeDestroy() {
      window.removeEventListener('resize', this.resizeWindow);
    },
    methods: {
      init: function(){
        var _this = this;
        window.sheet.tableInfoCB = function(row_len, col_len){
          _this.row_len = row_len;
          _this.col_len = col_len;
          _this.colnames = _this.getColnames(col_len);
          console.log(row_len);
          console.log(col_len);
          // debugger; // eslint-disable-line no-debugger
          var util_grid = window.util_grid;
          var _columns = util_grid.getColInfos(col_len);
          let columns = util_grid.initColHeader(_columns);
          var options = {
            columnPicker: {
                columnTitle: "Columns",
            },
            editable: true,
            enableAddRow: true,
            enableCellNavigation: true,
            enableColumnReorder: true,
            asyncEditorLoading: false,
            autoEdit: false,
            rowHeight: 24,
            editCommandHandler: function(item, column, editCommand){
              _this.cellEditDone(item, column, editCommand);
            }
          };
          var Slick = window.Slick;
          var remoteModel = new Slick.Data.RemoteModel();
          remoteModel.data.length = row_len;
          window.sheet.remoteModel = remoteModel;
          
          const $el = _this.$refs.mainGrid;
          var grid = new Slick.Grid($el, remoteModel.data, columns, options);
          _this.grid = grid;
          // grid.setSelectionModel(new Slick.CellSelectionModel());
          var onViewportChanged = function(){
            // debugger; // eslint-disable-line no-debugger
            var vp = grid.getViewport();
            window.sheet.remoteModel.ensureData(vp.top, vp.bottom);
          }
          var onScroll = window._.debounce(onViewportChanged, 250);
          grid.onViewportChanged.subscribe(onScroll);
          grid.onSort.subscribe(onViewportChanged);
          remoteModel.onDataLoaded.subscribe(function (e, args) {
            for (var i = args.from; i <= args.to; i++) {
              grid.invalidateRow(i);
            }
            grid.updateRowCount();
            grid.render();
          });
          // load the first page
          grid.onViewportChanged.notify();

          window.addEventListener('resize', _this.resizeWindow);
        }
        var common = window.common;
        common.getTableInfo('sheet.tableInfoCB');
      },
      getColnames: function(col_len){
        // debugger; // eslint-disable-line no-debugger 
        var common = window.common;
        var colnames = [];
        for(var i=0; i< col_len; i++){
          colnames.push(common.toBase26(i));
        }
        return colnames;
      },
      resizeWindow: function(event) {
        console.log(event);
        // debugger; // eslint-disable-line no-debugger
        const $mainGrid = $(this.$refs.mainGrid);
        const $viewport = $(this.$refs.viewport);
        this.resizeToFitBrowserWindow(this.grid, $mainGrid, $viewport);
      },
      calculateGridNewDimensions: function($grid, $viewport) {
        const DATAGRID_MIN_HEIGHT = 180;
        const DATAGRID_MIN_WIDTH = 300;
        const DATAGRID_BOTTOM_PADDING = 26;
        var availableHeight = $(window).height() - $grid.offsetTop - DATAGRID_BOTTOM_PADDING;
        var availableWidth = $viewport.width();
        var newHeight = availableHeight;
        var newWidth = availableWidth;
        // we want to keep a minimum datagrid size, apply these minimum if required
        if (newHeight < DATAGRID_MIN_HEIGHT) {
          newHeight = DATAGRID_MIN_HEIGHT;
        }
        if (newWidth < DATAGRID_MIN_WIDTH) {
          newWidth = DATAGRID_MIN_WIDTH;
        }
        return {
          height: newHeight,
          width: newWidth
        };
      },
      /** resize the datagrid to fit the browser height & width */
      resizeToFitBrowserWindow: function(grid, $mainGrid, $viewport) {
        // calculate new available sizes but with minimum height of 220px
        var newSizes = this.calculateGridNewDimensions($mainGrid, $viewport);
        if (newSizes) {
          // apply these new height/width to the datagrid
          $mainGrid.height(newSizes.height);
          $mainGrid.width(newSizes.width);
          // resize the slickgrid canvas on all browser except some IE versions
          // exclude all IE below IE11
          if (new RegExp('MSIE [6-8]').exec(navigator.userAgent) === null && grid) {
            grid.resizeCanvas();
          }
        }
      },
      cellEditDone: function(item, column, editCommand){
        let rowID = editCommand.row + 1;
        let colName = column.name;
        let oldValue = editCommand.prevSerializedValue;
        let newValue = editCommand.serializedValue;
        var common = window.common;
        common.cellEditDone(rowID, colName, oldValue, newValue);
        window.sheet.remoteModel.updateData(editCommand.row , colName, newValue);
        this.redrawView();
      },
      redrawView: function(){
        var vp = this.grid.getViewport();
        window.sheet.remoteModel.ensureData(vp.top, vp.bottom);
      }
    },
    created : function(){
    }
  }
</script>
