// http://cwestblog.com/2013/09/05/javascript-snippet-convert-number-to-column-name/
var __colNameList = [];
var __maxColCount = 16384;
function _prepareColName(){
  var index = 1;
  for(index = 1; index <= __maxColCount; index++){
    var num = index;
    for (var ret = '', a = 1, b = 26; (num -= a) >= 0; a = b, b *= 26) {
      ret = String.fromCharCode(parseInt((num % b) / a) + 65) + ret;
    }
     __colNameList.push(ret);
  }
}

_prepareColName();

export default{
  getColName(num){
    return __colNameList[num-1];
  },
  getColNames(){
    return __colNameList;
  },
  // http://cwestblog.com/2013/09/04/javascript-snippet-parse-a-spreadsheet-address/
  parseAddress(strCellAddr) {
    var i = strCellAddr.search(/\d/);
    var colNum = 0;
    strCellAddr = +strCellAddr.replace(/\D/g, function(letter) {
      colNum += (parseInt(letter, 36) - 9) * Math.pow(26, --i);
      return '';
    });
    return [strCellAddr, colNum];
  },
  getColInfos(col_count){
    var Slick = window.Slick;
    var columns = [
          {id: "id", name: "id", field: "id", cssClass: "grid-row-hader", selectable: false}
      ];
    for(var index = 0; index < col_count; index++){
      var colname = this.getColName(index+1);
      var item = {id: colname, name: colname, field: colname, editor: Slick.Editors.Text};
      columns.push(item);
    }
    return columns;
  },
  initColHeader(columns){
    // for (var i = 0; i < columns.length; i++) {
    //   columns[i].header = {
    //     menu: {
    //       items: [
    //         {
    //           iconImage: "../images/sort-asc.gif",
    //           title: "Sort Ascending",
    //           disabled: !columns[i].sortable,
    //           command: "sort-asc"
    //         },
    //         {
    //           iconImage: "../images/sort-desc.gif",
    //           title: "Sort Descending",
    //           disabled: !columns[i].sortable,
    //           command: "sort-desc"
    //         },
    //         {
    //           title: "Hide Column",
    //           command: "hide",
    //           tooltip: "Can't hide this column"
    //         },
    //         {
    //           divider: true,
    //           command: ""
    //         },
    //         {
    //           iconCssClass: "icon-help",
    //           title: "Help",
    //           command: "help"
    //         }
    //       ]
    //     }
    //   };
    // }
  
    return columns;
  }
}