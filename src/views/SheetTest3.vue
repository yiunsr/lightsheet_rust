<style>
.slick-cell.grid-row-hader{
  background: #f0f0f0;
}
.slick-row.active .slick-cell{
  background-color: #faebd7;
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

        var grid;
        var columns = [
          {id: "title", name: "Title", field: "title"},
          {id: "duration", name: "Duration", field: "duration"},
          {id: "%", name: "% Complete", field: "percentComplete"},
          {id: "start", name: "Start", field: "start"},
          {id: "finish", name: "Finish", field: "finish"},
          {id: "effort-driven", name: "Effort Driven", field: "effortDriven"},
          {id: "data1", name: "data1", field: "data1"},
          {id: "data2", name: "data2", field: "data2"},
          {id: "data3", name: "data3", field: "data3"},
          {id: "data4", name: "data4", field: "data4"},
          {id: "data5", name: "data5", field: "data5"},
          {id: "data6", name: "data6", field: "data6"},
          {id: "data7", name: "data7", field: "data7"},
          {id: "data8", name: "data8", field: "data8"},
          {id: "data9", name: "data9", field: "data9"},
          {id: "dataA", name: "dataA", field: "dataA"},
          {id: "dataB", name: "dataB", field: "dataB"},
          {id: "dataC", name: "dataC", field: "dataC"},
          {id: "dataD", name: "dataD", field: "dataD"},
          {id: "dataE", name: "dataE", field: "dataE"},
        ];

          var options = {
            enableCellNavigation: true,
            enableColumnReorder: false
          };

        var data = [];
        for (var i = 0; i <2000000; i++) {
          data[i] = {
            title: "Task " + i,
            duration: "5 days",
            percentComplete: Math.round(Math.random() * 100),
            start: "01/01/2009",
            finish: "01/05/2009",
            effortDriven: (i % 5 == 0),
            data1: i, data2: i, data3: i, data4: i,
            data5: i, data6: i, data7: i, data8: i,
            data9: i, dataA: i, dataB: i, dataC: i,
            dataD: i, dataE: i,
          };
        }

        var Slick = window.Slick;
        const $el = _this.$refs.mainGrid;
        grid = new Slick.Grid($el, data, columns, options);
        window.grid = grid;

      },
    },
    created : function(){
    }
  }
</script>
