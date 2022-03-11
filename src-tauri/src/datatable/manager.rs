use std::fs;
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::{Mutex, Once};
use std::time::Instant;
use std::collections::HashMap;
use lazy_static::lazy_static;
use rusqlite::Connection;
// use dashmap::DashMap;

use crate::datatable::csv_reader;
use crate::datatable::table_info::TableInfo;


pub struct TableManager{
    conn:Option<Connection>,
    table_hm: HashMap<u32, TableManagerItem>
}

pub struct TableManagerItem{
    table_name: String,
    row_len: u32,
    col_len: u32,
}

impl TableManager {
    // Another static method, taking two arguments:
    pub fn new() -> TableManager
    {
        let db_path = TableManager::get_temp_file_fath();
	    // 딱히 삭제가 안되더라도 무시한다. 
	    let _ = fs::remove_file(db_path.clone());
        // 메모리 데이터베이스
        // let conn = Connection::open_in_memory()?;
        let conn = Connection::open(db_path.clone()).unwrap();

        let _ = conn.pragma_update(None, "synchronous", &"OFF".to_string());
        let _ = conn.pragma_update(None, "journal_mode", &"MEMORY".to_string());
        let _ = conn.pragma_update(None, "cache_size", &"16000000".to_string());
        let _ = conn.pragma_update(None, "locking_mode", &"EXCLUSIVE".to_string());
        let _ = conn.pragma_update(None, "temp_store", &"MEMORY".to_string());
        // // https://blog.devart.com/increasing-sqlite-performance.html

        let table_hm = HashMap::<u32, TableManagerItem>::new();
        TableManager { conn: Some(conn), table_hm: table_hm }
    }

    pub fn open<F>(&mut self, window_id:u32, filepath:String, cb:F)
        where F: Fn(u32) -> ()
    {
        // let mut temp_path = env::temp_dir();
        // temp_path.push("tmp.db");
        // let db_path = temp_path.into_os_string().into_string().unwrap();
        let now = Instant::now();
        let table_name = self.get_table_name(window_id);
        let conn = self.conn.as_mut().unwrap();
        let table_info = csv_reader::read_csv(conn, filepath, table_name.clone(), cb);
        if let Err(e) = table_info{
            println!("error running example: {}", e);    
            return;
        }
        let table_info = table_info.unwrap();
        let spendtime = now.elapsed().as_secs_f64();
        println!("spendtime : {}", spendtime);
        println!("1s per insert : {}", (table_info.row_len as f64) / spendtime);
        let row_len = table_info.row_len;
        let col_len = table_info.col_len;
        self.table_hm.insert(window_id, TableManagerItem{
            table_name: table_name,
            row_len: row_len,
            col_len: col_len,
        });
    }

    pub fn get_temp_file_fath() -> String{
        "./tmp.db".to_string()
    }

    pub fn get_table_name(&self, window_id:u32) -> String{
        let mut table_name = "datatbl_".to_string();
        table_name.push_str(&window_id.to_string());
        table_name
    }

    pub fn get_row_len(&self, window_id:u32) -> u32{
        let tlb_item = self.table_hm.get(&window_id);
        tlb_item.unwrap().row_len
    }
    
    pub fn get_col_len(&self, window_id:u32) -> u32{
        let tlb_item = self.table_hm.get(&window_id);
        tlb_item.unwrap().col_len
    }
    
    pub fn get_rows(&mut self, window_id:u32, from:u32, to:u32) -> String{
        let table_name = self.get_table_name(window_id);
        let col_len = self.get_col_len(window_id);
        let conn = self.conn.as_mut().unwrap();
        csv_reader::get_rows(&conn, &table_name, 
            col_len, from, to)
    }

    pub fn cell_edit(&mut self, window_id:u32, row_id:u32, col_index:u32,
            old_value: String, new_value: String) -> bool{
        let table_name = self.get_table_name(window_id);
        let conn = self.conn.as_mut().unwrap();
        csv_reader::cell_edit(conn, &table_name, row_id, col_index, &old_value, &new_value)
    }

}

