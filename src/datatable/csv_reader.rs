use std::io::{Seek, SeekFrom};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str;
use log;
use chardetng::EncodingDetector;
use encoding_rs_io::DecodeReaderBytesBuilder;

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

pub fn csv_open(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
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
    
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(sep)
        .from_reader(transcoded);
    let mut counter = 0;
    for result in rdr.records() {
        let _ = skip_fail!(result);
        counter += 1;
        if counter % 50000 == 0{
            println!("x = {}", counter);

        }
    }
    Ok(())
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