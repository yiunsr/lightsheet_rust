use std::io;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::str;
use std::boxed::Box;
use std::convert::TryFrom;
use csv::Reader;
use log;
use chardetng::EncodingDetector;
use encoding_rs_io::DecodeReaderBytes;
use encoding_rs_io::DecodeReaderBytesBuilder;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;

use super::db_utils;

// pub type Callback = fn(u32);
pub struct TableInfo {
    pub conn: Connection,
	pub table_name: String,
	pub col_len: u32,
	pub row_len: u32,
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
		Result< (Reader<DecodeReaderBytes<&mut File, Vec<u8>>>, u32), Box<dyn Error>> {
    
    let mut buffer = [0; 1024*10];

    file.read(&mut buffer)?;
	file.seek(SeekFrom::Start(0))?;
	

    let mut det = EncodingDetector::new();
    det.feed(&buffer, true);
    let enc = det.guess(None, false);
    log::info!("file characterset detect: {}", enc.name());

    let str_buffer = String::from_utf8_lossy(&buffer);
	let sep = get_col_sep(&str_buffer);
	let col_count = get_col_count(&str_buffer, sep);
    
    // let transcoded = DecodeReaderBytesBuilder::new()
    //     .encoding(Some(enc))
	//     .build(file);

	let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(enc))
		//.build_with_buffer(file, vec![0; 1024 * (1 << 10)]).unwrap();
		.build_with_buffer(file, vec![0; 1024 * 4]).unwrap();
    
    let rdr = csv::ReaderBuilder::new()
        .delimiter(sep)
        .from_reader(transcoded);
	Ok((rdr, col_count))
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

// fn read_csv(dbfile: String, csvfile: String, cb:Callback
// 		) -> Result<Vec<Option<StringRecord>>, Box<dyn Error>> {
pub fn read_csv<F>(dbfile: String, csvfile: String, cb:F) ->Result<(TableInfo), Box<dyn Error>>
	where F: Fn(u32) -> ()
{

    // Read CSV File
    
    // 딱히 삭제가 안되더라도 무시한다. 
	fs::remove_file(dbfile.clone());
	
	let file= &mut File::open(csvfile).unwrap();
	//let file_pos = file.seek(SeekFrom::Current(0)).unwrap();
	let total_file_byte = file.metadata().unwrap().len();
	let (reader, col_count) = csv_open(file)?;
	let mut iter = reader.into_records();
	let mut old_percent = 0u32;

	// 메모리 데이터베이스
	// let conn = Connection::open_in_memory()?;
	let mut conn = Connection::open(dbfile.clone())?;
	conn.pragma_update(None, "synchronous", &"OFF".to_string());
	conn.pragma_update(None, "journal_mode", &"MEMORY".to_string());
	conn.pragma_update(None, "cache_size", &"10000".to_string());
	conn.pragma_update(None, "locking_mode", &"EXCLUSIVE".to_string());
	conn.pragma_update(None, "temp_store", &"MEMORY".to_string());
	// // https://blog.devart.com/increasing-sqlite-performance.html

	let table_name = "datatable".to_string();
	// let rec = records[0].unwrap();
	let c_sql = db_utils::create_query(&table_name, col_count);
	conn.execute(&c_sql, NO_PARAMS)?;

	let i_query = db_utils::insert_query(&table_name, col_count);
	let mut row_index:u32 = 1;

	let tx = conn.transaction()?;
	{
		let mut i_stmt = tx.prepare(&i_query)?;

		loop {
			// Read the position immediately before each record.
			// let next_pos = iter.reader().position().clone();
			let item = iter.next();
			if item.is_none() {
				break;
			}
			let mut fields:Vec<&str> = Vec::new();
			let record = item.unwrap()?;
			let row_idex_str = row_index.to_string().clone();
			fields.push(&row_idex_str);
			for field in record.iter() {
				fields.push(field);
			}
			row_index = row_index + 1;
			i_stmt.execute(fields)?;
			
			let reader_ = iter.reader_mut().get_mut();
			// reader.rdr.nread 에 접근해서 read 한 byte 를 접근할 수 있으면 좋겠다.
			
			// next_pos.byte() 는 100% 가 넘는 문제가 생김
			let nread = reader_.rdr.nread;
			// let read_byte = next_pos.byte();
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
	
	Ok(TableInfo {
		conn: conn,
		table_name: dbfile,
		col_len: col_count,
		row_len: row_index,
	})


	// elapsed := time.Since(start)
	// log.Printf("Total count :  %d", total_count)
	// log.Printf("Total spend Time :  %s", elapsed)
	// d2 := elapsed.Seconds()
	// log.Printf("1s insert :  %f", float64(total_count)/d2)
	// return db, table_name, records, total_count, col_count
}
