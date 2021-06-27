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
    table_name: String,
    row_len: u32,
    col_len: u32,
    table_id: u32,
}

static mut CONN:Option<Arc<Connection>> = None;

pub fn get_conn<'a>(table_id:u32)-> &'a Connection {
    let conn:&'a Connection;
    unsafe{
        conn = CONN.as_ref().unwrap();
    }
    conn
}


impl TableManager {
    // Another static method, taking two arguments:
    pub fn new(filepath:String) -> TableManager
    {
        TableManager{
            table_name: "".to_string(),
            row_len: 0, col_len: 0,
            table_id: 1}
    }

    pub fn open<F>(&mut self, filepath:String, cb:F) -> Option<TableManager>  
        where F: Fn(u32) -> ()
    {
        // let mut temp_path = env::temp_dir();
        // temp_path.push("tmp.db");
        // let db_path = temp_path.into_os_string().into_string().unwrap();
        let db_path = TableManager::get_temp_file_lath();
        let now = Instant::now();
        let table_info = csv_reader::read_csv(db_path, filepath, cb);
        if let Err(e) = table_info{
            println!("error running example: {}", e);    
            return None;
        }
        let table_info = table_info.unwrap();
        let spendtime = now.elapsed().as_secs_f64();
        println!("spendtime : {}", spendtime);
        println!("1s per insert : {}", (table_info.row_len as f64) / spendtime);
        unsafe{
            CONN = Some(table_info.conn);
        }
        self.table_name = table_info.table_name.clone();
        self.row_len = table_info.row_len;
        self.col_len = table_info.col_len;
        self.table_id = 1;

        
        Some(TableManager{
            table_name: table_info.table_name,
            row_len: table_info.row_len, col_len:table_info.col_len,
            table_id: 1u32
        })
    }

    pub fn get_temp_file_lath() -> String{
        "./tmp.db".to_string()
    }

    pub fn get_table_name(&self) -> String{
        self.table_name.clone()
    }

    pub fn get_row_len(&self) -> u32{
        self.row_len
    }
    
    pub fn get_col_len(&self) -> u32{
        self.col_len
    }
    
    pub fn get_rows(&self, from:u32, to:u32) -> String{
        let conn = get_conn(1u32);
        csv_reader::get_rows(&conn, &self.table_name, self.col_len, from, to)
    } 
    

}

