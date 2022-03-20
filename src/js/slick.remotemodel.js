if (typeof jQuery === "undefined") {
  throw new Error("SlickGrid requires jquery module to be loaded");
}


(function ($) {
  /***
  * A sample AJAX data store implementation.
  * Right now, it's hooked up to load search results from Octopart, but can
  * easily be extended to support any JSONP-compatible backend that accepts paging parameters.
  */
  function RemoteModel() {
    // private
    var Slick = window.Slick;
    var data = {length: 0};
    var searchstr = "";  // eslint-disable-line no-unused-vars
    var sortcol = null;  // eslint-disable-line no-unused-vars
    var sortdir = 1;  // eslint-disable-line no-unused-vars

    // events
    var onDataLoading = new Slick.Event();
    var onDataLoaded = new Slick.Event();
    function init() {
    }
    
    
    function isDataLoaded(from, to) {
      for (var i = from; i <= to; i++) {
        if (data[i] == undefined || data[i] == null) {
          return false;
        }
      }
  
      return true;
    }
  
  
    function clear() {
      for (var key in data) {
        delete data[key];
      }
      data.length = 0;
    }

    function ensureData(from, to) {   
      // debugger; // eslint-disable-line no-debugger
      const marginTop = 100;
      const marginBottom = 100;
      if (to == -1){
        to = from + 100;
      }
      from = Math.max(0, from - marginTop);
      to = to + marginBottom;

      var common = window.common;
      window.temp.cbRowInfo = function(tableData){
        for (var i = 0; i <= Object.keys(data).length; i++)
          delete data[i];
        var rows = tableData.values;
        for (var row_index in rows) {
          var data_index = parseInt(row_index);
          var row_dict = rows[data_index];
          var item = { id: row_dict["id"] };

          for(var colname in row_dict){
            if(colname == "id") continue;
            var colindex = parseInt(colname.slice(2,20))
            var base26 = common.toBase26(colindex);
            item[base26] = row_dict[colname];
        }
          data[from + data_index] = item;
          data[from + data_index].index = from + data_index;
        }
        setTimeout(function() {
          // debugger; // eslint-disable-line no-debugger 
          onDataLoaded.notify({from: from, to: to});
        }, 10);
      }
      common.getRows(from, to, "window.temp.cbRowInfo");
    }
    
    function reloadData(from, to) {
      for (var i = from; i <= to; i++)
        delete data[i];

      ensureData(from, to);
    }
  
    function setSort(column, dir) {
      sortcol = column;
      sortdir = dir;
      clear();
    }
  
    function setSearch(str) {
      searchstr = str;
      clear();
    }
    
    function updateData(rowID, col_name, value){
      data[rowID][col_name] = value;
      onDataLoaded.notify({from: rowID, to: rowID});
    }
  
    init();
  
    return {
      // properties
      "data": data,

      // methods
      "clear": clear,
      "isDataLoaded": isDataLoaded,
      "ensureData": ensureData,
      "reloadData": reloadData,
      "setSort": setSort,
      "setSearch": setSearch,
      "updateData": updateData,

      // events
      "onDataLoading": onDataLoading,
      "onDataLoaded": onDataLoaded
    };
  }
  
  // exports
  $.extend(true, window, { 
    Slick: { 
      Data: { 
        RemoteModel: RemoteModel 
      }
    }
  });
}(window.jQuery));