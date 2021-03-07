use std::env;
use std::time::Instant;
use std::sync::Arc;
use std::cell::RefCell;

use crate::datatable::csv_reader;
use rusqlite::Connection;

// static mut TABLE_INFO:Option<csv_reader::TableInfo> = None;
static mut TABLE_NAME:String = String::from("");
static mut ROW_LEN:u32 = 0;
static mut COL_LEN:u32 = 0;
static mut CONN:Option<rusqlite::Connection> = None;
static mut PERCENT:Option<u32> = None;

pub fn open_file<F>(filepath:String, cb:F)
    where F: Fn(u32) ->()
{

    unsafe{
        PERCENT = Some(0u32);
    }
    let mut temp_path = env::temp_dir();
    temp_path.push("tmp.db");
    let db_path = temp_path.into_os_string().into_string().unwrap();
    let now = Instant::now();
    let table_info = match csv_reader::read_csv(db_path, filepath, cb){
        Ok(ti) => ti,
        Err(error) => {
            println!("error running example: {}", error);    
            return;
        }
    };
    let spendtime = now.elapsed().as_secs_f64();
    println!("spendtime : {}", spendtime);
    println!("1s per insert : {}", (table_info.row_len as f64) / spendtime);
    unsafe{
        TABLE_NAME = table_info.table_name;
        ROW_LEN = table_info.row_len;
        COL_LEN = table_info.col_len;
        CONN = Some(table_info.conn);
        // TABLE_INFO = Some(table_info);
    }
}

fn callback(percent:u32){
    unsafe{
        PERCENT = Some(percent);
    }
    println!("percent : {}%", percent);
}

pub fn open_file_percent() ->u32 {
    let percent:u32;
    unsafe{
        percent = PERCENT.unwrap();
    }
    percent
}

pub fn get_table_name() -> String{
    let table_name:String ;
    unsafe{
        table_name = TABLE_NAME;
    }
    table_name
}

pub fn get_row_len() -> u32{
    let row_len:u32;
    unsafe{
        row_len = ROW_LEN;
    }
    row_len
}

pub fn get_col_len() -> u32{
    let col_len:u32;
    unsafe{
        col_len = COL_LEN;
    }
    col_len
}


pub fn get_conn() -> &'static rusqlite::Connection {
    let conn:rusqlite::Connection;
    unsafe{
        conn = CONN.unwrap();
    }
    &conn
}

pub fn get_rows(from:u32, to:u32) ->String{
    let table_name = get_table_name();
    let col_len = get_col_len();
    let conn = get_conn();
    csv_reader::get_rows(conn, table_name, col_len, from, to)
} 
