<template>
  <div>
    <div ref="mainGrid" style="width:100%;height:calc(100vh - 36px - 24px);"></div>
  </div>
</template>

<script>
  require('../../node_modules/slickgrid/slick.grid.css');
  require('../../node_modules/slickgrid/slick-default-theme.css');
  require('../assets/plugins-gdoc-style.css');

  global.jQuery = require('jquery');
  var $ = global.jQuery;
  window.$ = $;
  require('jquery-ui');
  require('../../node_modules/slickgrid/lib/jquery.event.drag-2.3.0.js');
  require('../../node_modules/slickgrid/lib/jquery.event.drop-2.3.0.js');
  // debugger; // eslint-disable-line no-debugger
  require('../../node_modules/slickgrid/slick.core.js');
  require('../../node_modules/slickgrid/slick.editors.js');
  require('../../node_modules/slickgrid/slick.grid.js');
  require('../../node_modules/slickgrid/slick.grid.js');
  require('../js/slick.remotemodel.js');

  export default {
    name: 'GridSheet',
    data: () => ({}),
    mounted: function(){
        // debugger; // eslint-disable-line no-debugger
      var Slick = window.Slick;
      var columns = [
        {id: "title", name: "Title", field: "title", editor: Slick.Editors.Text},
        {id: "duration", name: "Duration", field: "duration", editor: Slick.Editors.Text},
        {id: "%", name: "% Complete", field: "percentComplete", editor: Slick.Editors.Text},
        {id: "start", name: "Start", field: "start", editor: Slick.Editors.Text},
        {id: "finish", name: "Finish", field: "finish", editor: Slick.Editors.Text},
        {id: "effort-driven", name: "Effort Driven", field: "effortDriven", editor: Slick.Editors.Text}
      ];

      var options = {
        enableCellNavigation: true,
        enableColumnReorder: false,
        editable: true,
        enableAddRow: false,
        autoHeight: false,
      };

      var data = [];
      for (var i = 0; i < 2000000; i++) {
        data[i] = {
          title: "Task " + i,
          duration: "5 days",
          percentComplete: Math.round(Math.random() * 100),
          start: "01/01/2009",
          finish: "01/05/2009",
          effortDriven: (i % 5 == 0)
        };
      }

      const $el = this.$refs.mainGrid;
      new window.Slick.Grid($el, data, columns, options);
      
    },
    methods: {
    },
    
    beforeCreate: function(){
    }
  }
</script>
