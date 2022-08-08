use std::collections::HashMap;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use rusqlite::params;
use serde_json::Number;
use serde_json::Value;
use regex::Regex;

use super::db_utils;


pub fn new_table(conn:&mut rusqlite::Connection, window_id:u32,
	row_len: u32, col_len: u32)
{
	let tx = conn.transaction().unwrap();

	{
		let c_sql = db_utils::create_query_main(window_id, col_len, false);
		let _ = tx.execute(&c_sql, NO_PARAMS);
	}

	{
		let i_query = db_utils::insert_query(window_id, col_len);
		let mut i_stmt = tx.prepare(&i_query).unwrap();

		for row_id in 1..row_len + 1{
			let _ = i_stmt.raw_bind_parameter(1 as usize, row_id);	
			for col_idx in 1..col_len + 1{
				let _ = i_stmt.raw_bind_parameter((col_idx+1) as usize, "");	
			}
			let _= i_stmt.raw_execute();
		}
	}

	{
		let c_sql = db_utils::create_query_rowmeta(window_id);
		let _ = tx.execute(&c_sql, NO_PARAMS);
	}

	{
		let c_sql = db_utils::insert_query_rowmeta(window_id);
		let _ = tx.execute(&c_sql, NO_PARAMS);
	}

	tx.commit().unwrap();
}

pub fn get_rows(conn:&rusqlite::Connection, window_id:u32, 
	col_len: u32, from: u32, to: u32) -> String
{
	let blank1 = String::from("");
	let blank2 = String::from("");
	let blank3 = String::from("");
	let where_q = "row_idx >= ?1 and row_idx <= ?2".to_string();
	let sql = db_utils::select_query(window_id, col_len, where_q, blank2, blank3);
	let mut stmt = conn.prepare(&sql).unwrap();
	let mut data_dict:HashMap<String, Value> = HashMap::new();
	let mut row_slice:Vec<Value> = Vec::<Value>::with_capacity(100);
	let mut rows = stmt.query(params![from, to]).unwrap();
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

pub fn add_rows(conn:&mut Connection, window_id:u32, row_idx:u32, 
	row_add_count:u32, row_len:u32, col_len:u32)
{
	println!("== add_rows ==");
	println!("add_rows({}, {}, {})",row_idx, row_add_count, row_len);
	let tx = conn.transaction().unwrap();
	{
		let c_sql = db_utils::move_for_add_rows_query_rowmeta(window_id, row_idx, row_add_count) ;
		println!("{}", c_sql);
		let _ = tx.execute(&c_sql, NO_PARAMS);
	}
	{
		let c_sql_main = db_utils::insert_blank_query(window_id, col_len);
		println!("{}", c_sql_main);
		for row_i in 0..row_add_count{
			let _ = tx.execute(&c_sql_main, NO_PARAMS);
		}
	}
	{
		let c_sql_row_meta = db_utils::insert_blank_query_rowmeta(window_id);
		println!("{}", c_sql_row_meta);
		let mut i_stmt = tx.prepare(&c_sql_row_meta).unwrap();
		for row_i in 0..row_add_count{
			i_stmt.raw_bind_parameter(1 as usize, (row_len + row_i + 1) as i32).unwrap();
			i_stmt.raw_bind_parameter(2 as usize, (row_idx + row_i) as i32).unwrap();
			println!("{}, {}", row_len + row_i + 1, row_idx + row_i);
			let _= i_stmt.raw_execute();
		}
	}
	tx.commit().unwrap();
}

pub fn cell_edit(conn:&mut Connection, window_id:u32,
	row_id:u32, col_index:u32, old_value:&String, new_value:&String) -> bool
{
	let main_table = db_utils::get_table_name(window_id, db_utils::TableType::MainTable);
	let meta_table = db_utils::get_table_name(window_id, db_utils::TableType::RowMeta);
	let re = Regex::new("[\"]").unwrap();
	let double_quoted_new_value = re.replace_all(new_value, "\"\"");

	let col_name = &db_utils::colname(col_index).to_owned();
	let mut query = String::from("UPDATE ");
	query.push_str(&main_table);
	query.push_str(" SET ");
	query.push_str(col_name);
	query.push_str(" = \"");
	query.push_str(&double_quoted_new_value);
	query.push_str("\" WHERE id IN (");

	query.push_str("SELECT row_meta_id FROM ");
	query.push_str(&meta_table);
	query.push_str(" WHERE row_idx = ");
	query.push_str(&row_id.to_string());
	query.push_str(");");
	
	println!("{}", query);

	let tx = conn.transaction().unwrap();
	tx.execute(&query, NO_PARAMS).unwrap();
	tx.commit().unwrap();
	true
}
