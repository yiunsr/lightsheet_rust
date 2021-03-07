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
      debugger; // eslint-disable-line no-debugger 
      if (to == -1){
        to = from + 100;
      }

      var common = window.common;
      common.get_rows(from, to)
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