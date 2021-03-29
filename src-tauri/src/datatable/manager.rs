use std::sync::Arc;
use std::sync::{Mutex, Once};
use std::time::Instant;
use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::datatable::csv_reader;
use rusqlite::Connection;

pub struct TableManager{
    table_name: String,
    row_len: u32,
    col_len: u32,
    table_id: u32,
}

// https://stackoverflow.com/a/38905082
lazy_static! {
    static ref DbConnectionPool: HashMap<u32, &'static Connection> = {
        let mut m = HashMap::new();
        m
    };
}


// fn get_connection_pool_mg() -> &'static HashMap<u32, Connection> {
//     lazy_static! {
//         static ref DbConnectionPool: HashMap<u32, Connection> = HashMap::new();
//     }
//     &DbConnectionPool
// }

fn get_connection(table_id:u32)-> &'static Connection {
    DbConnectionPool.get(&table_id).unwrap()
}

fn set_connection(table_id:u32, conn:&'static Connection){
    DbConnectionPool.insert(table_id, conn);
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

    pub fn open<F>(&self, filepath:String, cb:F) -> Option<TableManager>  
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
        set_connection(1u32, &table_info.conn);
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
        let conn = get_connection(1u32);
        csv_reader::get_rows(&conn, &self.table_name, self.col_len, from, to)
    } 
    

}

