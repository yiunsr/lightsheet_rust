// http://cwestblog.com/2013/09/05/javascript-snippet-convert-number-to-column-name/
var __colNameList = [];
var __maxColCount = 4096;
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
  getColIndx(colname){
    var base = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', i, j, result = 0;
    for (i = 0, j = colname.length - 1; i < colname.length; i += 1, j -= 1) {
      result += Math.pow(base.length, j) * (base.indexOf(colname[i]) + 1);
    }
  
    return result;
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
          {id: "id", name: "", field: "id", cssClass: "grid-row-hader", selectable: false}
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