use std::env;
use std::time::Instant;

use crate::datatable::csv_reader;
use rusqlite::Connection;

pub struct TableManager{
    table_name: String,
    row_len: u32,
    col_len: u32,
    conn: Connection,
}

impl TableManager {

    // Another static method, taking two arguments:
    pub fn new<F>(filepath:String, cb:F) -> Option<TableManager>  
        where F: Fn(u32) -> ()
    {
        let mut temp_path = env::temp_dir();
        temp_path.push("tmp.db");
        let db_path = temp_path.into_os_string().into_string().unwrap();
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
        Some(TableManager{
            table_name: table_info.table_name,
            row_len: table_info.row_len, col_len:table_info.col_len,
            conn: table_info.conn})
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
    
    pub fn get_rows(&self, from:u32, to:u32) ->String{
        csv_reader::get_rows(&self.conn, &self.table_name, self.col_len, from, to);
        String::from("")
    } 
    

}

