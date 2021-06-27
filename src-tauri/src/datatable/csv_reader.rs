use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::str;
use std::boxed::Box;
use std::convert::TryFrom;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use csv::Reader;
use log;
use chardetng::EncodingDetector;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use serde_json::Number;
use unicode_bom::Bom;


use super::db_utils;
use super::table_info::TableInfo;

// // pub type Callback = fn(u32);
// pub struct TableInfo {
//     pub conn: Connection,
// 	pub table_name: String,
// 	pub col_len: u32,
// 	pub row_len: u32,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct RowInfo{
// 	values: HashMap<String, Value>,
// }


macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                println!("An error: {}; skipped.", e);
                continue;
            }
        }
    };
}

pub fn get_col_sep(readedstr:&str) -> u8 {
    let comma = readedstr.matches(',').count();
    let tab = readedstr.matches('\t').count();
    let pipe = readedstr.matches('|').count();
	if comma > tab && comma > pipe {
		return b',';
	}
	if tab > pipe {
		return b'\t';
	}
	b'|'
}

pub fn get_col_count(readedstr:&str, sep:u8) -> u32 {
	let mut lines = readedstr.lines();
	let line = lines.next().unwrap();
	let sep_count = line.matches(sep as char).count();
	sep_count as u32 + 1
}


pub fn read_csv<'conn, F>(dbfile: String, csvfile: String, cb:F) ->Result<TableInfo, Box<dyn Error>>
	where F: Fn(u32) -> ()
{

    // Read CSV File
	// 딱히 삭제가 안되더라도 무시한다. 
	fs::remove_file(dbfile.clone());
		
	let file= &mut File::open(csvfile).unwrap();
	//let file_pos = file.seek(SeekFrom::Current(0)).unwrap();
	let total_file_byte = file.metadata().unwrap().len();
	// let (mut reader, col_count) = csv_open(file)?;

	let mut buffer = [0; 1024*10];

	file.read(&mut buffer)?;
	file.seek(SeekFrom::Start(0))?;
	
	// BOM Check
	let bom: Bom = Bom::from(&buffer[0..4]);
	if Bom::Utf8 == bom {
		println!("Utf8");
	}

	let mut det = EncodingDetector::new();
	det.feed(&buffer, true);
	let enc = det.guess(None, false);
	log::info!("file characterset detect: {}", enc.name());

	let str_buffer = String::from_utf8_lossy(&buffer);
	let sep = get_col_sep(&str_buffer);
	let col_count = get_col_count(&str_buffer, sep);

	let mut rdr = csv::ReaderBuilder::new().delimiter(sep)
		.from_reader(file);
	//.delimiter(sep);
	//let iter = rdr.byte_records();

	let mut old_percent = 0u32;
	let ucol_count:usize = col_count as usize;

	// 메모리 데이터베이스
	// let conn = Connection::open_in_memory()?;
	let mut conn = Connection::open(dbfile.clone())?;
	conn.pragma_update(None, "synchronous", &"OFF".to_string());
	conn.pragma_update(None, "journal_mode", &"MEMORY".to_string());
	conn.pragma_update(None, "cache_size", &"10000".to_string());
	conn.pragma_update(None, "locking_mode", &"EXCLUSIVE".to_string());
	conn.pragma_update(None, "temp_store", &"MEMORY".to_string());
	// // https://blog.devart.com/increasing-sqlite-performance.html

	let table_name = "datatable_01".to_string();
	// let rec = records[0].unwrap();
	let c_sql = db_utils::create_query(&table_name, col_count);
	conn.execute(&c_sql, NO_PARAMS)?;

	let i_query = db_utils::insert_query(&table_name, col_count);
	let mut row_index:u32 = 1;
	let mut iter = rdr.into_byte_records();

	let tx = conn.transaction()?;
	{
		let mut i_stmt = tx.prepare(&i_query)?;

		loop {
			// Read the position immediately before each record.
			// let next_pos = iter.reader().position().clone();
			let opt_result_byterecord = iter.next();
			if opt_result_byterecord .is_none(){
				break;
			}
			let next_pos = iter.reader().position().clone();
			let record = opt_result_byterecord.unwrap().unwrap();

			//i_stmt.raw_bind_int32(1 as usize, row_index as i32)?;
			//i_stmt.raw_bind_parameter(1 as usize, row_index as i32)?;
			i_stmt.raw_bind_parameter(1 as usize, row_index as i32)?;			
			let mut col_index = 2;
			//let row_idex_str = row_index.to_string().clone();
			//fields.push(&row_idex_str);
			for field in record.iter() {
				let s= enc.decode_without_bom_handling_and_without_replacement(field).unwrap();
				//i_stmt.raw_bind_text_static(col_index, s.as_bytes())?;
				//i_stmt.raw_bind_parameter(col_index, s.as_bytes())?;
				i_stmt.raw_bind_parameter(col_index, s)?;
				col_index += 1;
			}
			//i_stmt.execute(params)
			row_index = row_index + 1;
			//i_stmt.execute_with_bound_parameters();
			i_stmt.raw_execute();
			
			let reader_ = iter.reader_mut().get_mut();
			// reader.rdr.nread 에 접근해서 read 한 byte 를 접근할 수 있으면 좋겠다.
			
			// next_pos.byte() 는 100% 가 넘는 문제가 생김
			let nread = next_pos.byte();
			let cur_percent:u32 = u32::try_from(nread as u64 * 100 / total_file_byte ).unwrap();
			if cur_percent != old_percent {
				if cur_percent < 100{
					cb(cur_percent);
				}
				else if cur_percent == 100{
					cb(cur_percent);
				}
				
			}
			old_percent = cur_percent;
		}
	}
	tx.commit()?;
	//conn.execute("Commit;", NO_PARAMS)?;

	println!("total row_index : {}", row_index);
	let table_info = TableInfo {
		conn: Arc::new(conn),
		table_name: table_name,
		col_len: col_count,
		row_len: row_index,
	};
	
	Ok(table_info)
}


pub fn get_rows(conn:&rusqlite::Connection, table_name:&String, 
	col_len: u32, from: u32, to: u32) -> String
{
	let from_ = from + 1;
	let blank1 = String::from("");
	let blank2 = String::from("");
	let blank3 = String::from("");
	let mut sql = db_utils::select_query(table_name, col_len, blank1, blank2, blank3);
	sql.push_str(" WHERE id >= ?1 limit 100;");
	let mut stmt = conn.prepare(&sql).unwrap();
	let mut data_dict:HashMap<String, Value> = HashMap::new();
	let mut row_slice:Vec<Value> = Vec::<Value>::with_capacity(100);
	let mut rows = stmt.query(params![from_]).unwrap();
	while let Some(row) = rows.next().unwrap() {
		let id_:u32 = row.get(0 as usize).unwrap();
		let mut item:HashMap<String, Value> = HashMap::new();
		item.entry(String::from("id")).or_insert(Value::Number(Number::from(id_)));
		print!("{}", id_);
        for i in 1..col_len+1{
			let value = Value::String(row.get_unwrap(i as usize));
			let colname = &db_utils::colname((i-1) as u32).to_owned();
			item.entry(colname.to_string()).or_insert(value);
		}
		let rowinfo = serde_json::to_value(item).unwrap();
		row_slice.push(rowinfo);
    }
	data_dict.entry(String::from("values")).or_insert(Value::Array(row_slice));
	let json_str = serde_json::to_string(&data_dict).unwrap();
	json_str
}