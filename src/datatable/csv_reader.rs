use std::io::{Seek, SeekFrom};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::str;
use std::boxed::Box;
use std::convert::TryFrom;
use csv::Reader;
use csv::StringRecord;
use csv::StringRecordsIter;
use log;
use chardetng::EncodingDetector;
use encoding_rs_io::DecodeReaderBytes;
use encoding_rs_io::DecodeReaderBytesBuilder;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;

use super::db_utils;

pub type Callback = fn(u32);
pub struct TableInfo {
    conn: Connection,
	table_name: String,
	col_len: u32,
	row_len: u32,
}

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

pub fn csv_open(file: &mut File) -> 
		Result<Reader<DecodeReaderBytes<&mut File, Vec<u8>>>, Box<dyn Error>> {
    
    let mut buffer = [0; 1024*10];

    file.read(&mut buffer)?;
    file.seek(SeekFrom::Start(0))?;

    let mut det = EncodingDetector::new();
    det.feed(&buffer, true);
    let enc = det.guess(None, false);
    log::info!("file characterset detect: {}", enc.name());

    let str_buffer = String::from_utf8_lossy(&buffer);
    let sep = get_col_sep(&str_buffer);
    
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(enc))
        .build(file);
    
    let rdr = csv::ReaderBuilder::new()
        .delimiter(sep)
        .from_reader(transcoded);
	Ok(rdr)
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

// fn read_csv(dbfile: String, csvfile: String, cb:Callback
// 		) -> Result<Vec<Option<StringRecord>>, Box<dyn Error>> {

//     // Read CSV File
    
//     // 딱히 삭제가 안되더라도 무시한다. 
// 	fs::remove_file(dbfile);
	
// 	let mut file = File::open(csvfile)?;
// 	let reader = csv_open(&file)?;
// 	let counter = 0u32;
// 	for result in reader.records() {
//         let _ = skip_fail!(result);
//         counter += 1;
//         if counter % 50000 == 0{
//             println!("x = {}", counter);

//         }
//     }

// 	let rec = records[0].unwrap();
// 	let col_count = u32::try_from(rec.len()).unwrap();

// 	// 메모리 데이터베이스
// 	// let conn = Connection::open_in_memory()?;
// 	let conn = Connection::open(dbfile)?;
// 	conn.execute("PRAGMA synchronous = OFF", NO_PARAMS);
// 	conn.execute("PRAGMA journal_mode = MEMORY", NO_PARAMS);
// 	conn.execute("PRAGMA cache_size = 10000", NO_PARAMS);
// 	conn.execute("PRAGMA locking_mode = EXCLUSIVE", NO_PARAMS);
// 	conn.execute("PRAGMA temp_store = MEMORY", NO_PARAMS);
// 	// // https://blog.devart.com/increasing-sqlite-performance.html

// 	let table_name = "datatable".to_string();
// 	let c_sql = db_utils::create_query(table_name, col_count);
// 	conn.execute(&c_sql, NO_PARAMS);

// 	let i_query = db_utils::insert_query(table_name, col_count);

// 	let total_count = records.len();
// 	let percent = 0u32;
// 	let newpercent = 0u32;
// 	let tx = conn.transaction()?;
// 	let mut stmt = tx.prepare(&i_query)?;
// 	stmt.execute(&["Joe Smith"])?;
	
// 	let row_index = 0;
// 	for record in records.iter() {
// 		newpercent = (row_index * 100) / total_count;
// 		if percent != newpercent {
// 			cb(newpercent);
// 			percent = newpercent;
// 		}
// 	interface_record := make([]interface{}, col_count+1)
// 	interface_record[0] = row_index + 1
// 	for i, v := range record {
// 		if i == col_count {
// 			break
// 		}
// 		interface_record[i+1] = v
// 	}
// 	for j := len(record); j < col_count; j++ {
// 		interface_record[j+1] = ""
// 	}

// 	tx.commit();
// 	// for row_index, record := range records {
// 	// 	newpercent = (row_index * 100) / total_count
// 	// 	if percent != newpercent {
// 	// 		cb(newpercent)
// 	// 		percent = newpercent
// 	// 	}
// 	// 	interface_record := make([]interface{}, col_count+1)
// 	// 	interface_record[0] = row_index + 1
// 	// 	for i, v := range record {
// 	// 		if i == col_count {
// 	// 			break
// 	// 		}
// 	// 		interface_record[i+1] = v
// 	// 	}
// 	// 	for j := len(record); j < col_count; j++ {
// 	// 		interface_record[j+1] = ""
// 	// 	}

// 	// 	err := stmt.Exec(interface_record...)
// 	// 	if err != nil {
// 	// 		log.Fatal(err)
// 	// 	}

// 	// }

// 	// db.Commit()

// 	// elapsed := time.Since(start)
// 	// log.Printf("Total count :  %d", total_count)
// 	// log.Printf("Total spend Time :  %s", elapsed)
// 	// d2 := elapsed.Seconds()
// 	// log.Printf("1s insert :  %f", float64(total_count)/d2)
// 	// return db, table_name, records, total_count, col_count
// }
