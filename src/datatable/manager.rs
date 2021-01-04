

use std::time::Instant;
use crate::datatable::csv_reader;

static mut TABLE_INFO:Option<csv_reader::TableInfo> = None;
static mut PERCENT:Option<u32> = None;

pub fn open_file<F>(filepath:String, cb:F)
    where F: Fn(u32) ->()
{

    unsafe{
        PERCENT = Some(0u32);
    }
    let db_path = "D:\\workspace\\vscode\\lightsheet_rust\\tmp\\tmp.db".to_string();
    let now = Instant::now();
    let table_info = match csv_reader::read_csv(db_path, filepath, callback){
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
        TABLE_INFO = Some(table_info);
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