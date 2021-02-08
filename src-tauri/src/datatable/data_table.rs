pub const DATABASE_PATH:&str="./data.db";

// pub fn OpenTableFile(filepath:String, cb:Callback) -> TableInfo {
// 	if curDB != nil {
// 		curDB.Close()
// 	}

// 	curDB, curTableName, curRecords, curRowCount, curColCount = ReadCSV("./data.db", filepath, cb)
// 	min := utils.MinInt(100, len(curRecords))

// 	var alter_query string
// 	alter_query = "ALTER TABLE " + curTableName + " ADD height INTEGER DEFAULT 24;"
// 	Exec(curDB, alter_query)

// 	// if row hide by some reason(filter_hide, user_hide) it get 0
// 	alter_query = "ALTER TABLE " + curTableName + " ADD actual_height INTEGER DEFAULT 24;"
// 	Exec(curDB, alter_query)

// 	alter_query = "ALTER TABLE " + curTableName + " ADD user_show INTEGER DEFAULT 1;" // 0 : hide,  1 : show
// 	Exec(curDB, alter_query)

// 	alter_query = "ALTER TABLE " + curTableName + " ADD filter_show INTEGER DEFAULT 1;" // 0 : hide by filter, 1 : show
// 	Exec(curDB, alter_query)

// 	return curDB, curTableName, curRecords[0:min], curRowCount, curColCount
// }
