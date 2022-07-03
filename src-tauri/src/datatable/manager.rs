use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use rusqlite::Connection;

use super::db_utils;
use crate::datatable::csv_reader;
use crate::datatable::table_info::TableInfo;


pub struct TableManager{
    conn:Option<Connection>,
    table_hm: HashMap<u32, TableManagerItem>
}

pub struct TableManagerItem{
    row_len: u32,
    col_len: u32,
}
impl TableManagerItem {
    fn set_row_len(&mut self, row_len: u32){
        self.row_len = row_len;
    }
    fn set_col_len(&mut self, col_len: u32){
        self.col_len = col_len;
    }
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
        let conn = self.conn.as_mut().unwrap();
        let table_info = csv_reader::read_csv(conn, filepath, window_id, cb);
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
            row_len: row_len,
            col_len: col_len,
        });
    }

    pub fn export_file<F>(&mut self, window_id:u32, filepath:String, cb:F)
        where F: Fn(u32) -> ()
    {
        let table_info = self.table_hm.get(&window_id).unwrap();
        let row_len = table_info.row_len;
        let col_len = table_info.col_len;

        let conn = self.conn.as_mut().unwrap();
        csv_reader::export_file(
            conn, window_id, row_len, col_len,  filepath, cb);
    }

    pub fn get_temp_file_fath() -> String{
        "./tmp.db".to_string()
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
        let col_len = self.get_col_len(window_id);
        let conn = self.conn.as_mut().unwrap();
        csv_reader::get_rows(&conn, window_id, 
            col_len, from, to)
    }

    pub fn add_rows(&mut self, window_id:u32, row_idx:u32, row_add_count:u32){
        let tlb_item = self.table_hm.get_mut(&window_id);
        let tlb_item = tlb_item.unwrap();
        let row_len = tlb_item.row_len;
        let col_len = tlb_item.col_len;
        let conn = self.conn.as_mut().unwrap();
        csv_reader::add_rows(conn, window_id, row_idx, row_add_count, row_len, col_len);
        tlb_item.set_row_len(row_len + row_add_count);
    }

    pub fn cell_edit(&mut self, window_id:u32, row_id:u32, col_index:u32,
            old_value: String, new_value: String) -> bool{
        let conn = self.conn.as_mut().unwrap();
        csv_reader::cell_edit(conn, window_id, row_id, col_index, &old_value, &new_value)
    }

}

