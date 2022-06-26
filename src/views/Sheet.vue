<style>
.slick-cell.grid-row-hader{
  background: #f0f0f0;
}
.slick-row .slick-cell.active{
  padding-top: 2px;
}

.slick-cell.col-active{
  background: #d4e7ff;
}
/* .slick-viewport-left{
  overflow: hidden !important;
} */


#contextMenu {
  background: #e1efc7;
  border: 1px solid gray;
  padding: 2px;
  display: inline-block;
  min-width: 100px;
  -moz-box-shadow: 2px 2px 2px silver;
  -webkit-box-shadow: 2px 2px 2px silver;
  z-index: 99999;
  margin-top: -30px;
}
</style>


<template>
  <div  ref="viewport">
    <div ref="mainGrid" style="width:100%;height:calc(100vh - 36px - 24px);"
      autofocus
    ></div>

     <v-menu
      v-model="showContextMenu"
      absolute
      :position-x="contextMenuLeft"
      :position-y="contextMenuTop"
    >

      <v-list dense>
        <v-list-item
          v-for="(item, index) in rowContextMenuItems" @click="selectMenu(item.menuID)"
          :key="'row' + index"
        >
          <v-list-item-title>{{ $i18n.t("context_menu." + item.title ) }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>
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
          mouseClickedCell: null,
          showContextMenu: false, contextMenuTop: 0, contextMenuLeft: 0,
          showRowContextMenu: false,
          rowContextMenuItems: [
            { title: 'add_row_above',  menuID: 'add_row_above'},
            { title: 'add_row_below',  menuID: 'add_row_below' },
          ],
          
          selCellCol: -1, selCellRow: -1,
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
            frozenColumn: 0,
            frozenRow: -1,
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
          window.grid = grid;
          _this.grid = grid;
          // grid.setSelectionModel(new Slick.CellSelectionModel());
          var onViewportChanged = function(){
            $(".slick-cell.r" + _this.selCellCol).addClass("col-active");
            var vp = grid.getViewport();
            window.sheet.remoteModel.ensureData(vp.top, vp.bottom);
          }
          var onScroll = window._.debounce(onViewportChanged, 250);
          grid.onViewportChanged.subscribe(function(){
            // debugger; // eslint-disable-line no-debugger
            $(".slick-cell.r" + _this.selCellCol).addClass("col-active");
            onScroll();
          });
          grid.onSort.subscribe(onViewportChanged);
          remoteModel.onDataLoaded.subscribe(function (e, args) {
            for (var i = args.from; i <= args.to; i++) {
              grid.invalidateRow(i);
            }
            grid.updateRowCount();
            // debugger; // eslint-disable-line no-debugger
            grid.render();
            $(".slick-cell.r" + _this.selCellCol).addClass("col-active");
          });
          remoteModel.onDataLoading.subscribe(function () {
            $(".slick-cell.r" + _this.selCellCol).addClass("col-active");
          });
          // load the first page
          grid.onViewportChanged.notify();
          window.addEventListener('resize', _this.resizeWindow);

          // mouse right click
          grid.onContextMenu.subscribe(function (e) {
            // debugger; // eslint-disable-line no-debugger
            e.preventDefault();
            let cell = grid.getCellFromEvent(e);
            grid.setActiveCell(cell.row, cell.cell);
            _this.mouseClickedCell = cell;
            _this.contextMenuTop = e.pageY;
            _this.contextMenuLeft = e.pageX;
            if(cell.cell == 0){
              _this.showContextMenu = true;
              _this.showRowContextMenu = true;
            }
          });

          grid.onHeaderClick.subscribe(function(e, args) {
            var colID = args.column.id;
            console.log(colID);
          });
          grid.onHeaderContextMenu.subscribe(function(e, args) {
            // debugger; // eslint-disable-line no-debugger
            let colID = args.column.id;
            let colIdx = util_grid.getColIndx(colID)
            console.log(colIdx);
          });
          grid.onActiveCellChanged.subscribe(function(e, args) {
            // debugger; // eslint-disable-line no-debugger
            _this.selCellCol = args.cell;
            _this.selCellRow = args.row;
            $(".col-active").removeClass("col-active");
            $(".slick-cell.r" + args.cell).addClass("col-active");
          });
          

          // debugger; // eslint-disable-line no-debugger
          // set focus A1
          grid.setActiveCell(0, 1);
        }

        $("body").one("click", function () {
          _this.showContextMenu = false;
          _this.showRowContextMenu = false;
        });
        var common = window.common;
        common.getTableInfo().then(function(res){
            window.sheet.tableInfoCB(res.row_len, res.col_len)
          });
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
        let rowID = item.id;
        let colName = column.name;
        let oldValue = editCommand.prevSerializedValue;
        let newValue = editCommand.serializedValue;
        var common = window.common;
        common.cellEditDone(rowID, colName, oldValue, newValue);
        console.log("row id : " + rowID);
        window.sheet.remoteModel.updateData(editCommand.row , colName, newValue);
        //this.redrawView();
      },
      redrawView: function(){
        var vp = this.grid.getViewport();
        window.sheet.remoteModel.ensureData(vp.top, vp.bottom);
      },
      selectMenu: function(menuID){
        // debugger; // eslint-disable-line no-debugger
        let common = window.common;
        switch(menuID){
          case "add_row_above":{
            common.addRows(this.selCellRow+1, 1);
            this.redrawView();
            break;
          }
          case "add_row_below":{
            common.addRows(this.selCellRow+2, 1);
            this.redrawView();
            break;
          }
          
        }
        window.sheet.remoteModel.data.length += 1;
      },
    },
    created : function(){
    }
  }
</script>
