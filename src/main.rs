use std::io::{Seek, SeekFrom};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process;
use std::time::Instant;
use chardetng::EncodingDetector;
use encoding_rs_io::DecodeReaderBytesBuilder;

mod datatable;

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

fn csv_open(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0; 1024];

    file.read(&mut buffer)?;
    file.seek(SeekFrom::Start(0))?;

    let mut det = EncodingDetector::new();
    det.feed(&buffer, true);
    let enc = det.guess(None, false);
    println!("file characterset detect: {}", enc.name());
    
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(enc))
        .build(file);
    
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
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

fn main() {
    let s = datatable::db_utils::colname(10);
    println!("{}", s);

    let file_path = String::from("D:\\workspace\\vscode2\\hello_cargo\\res\\경상남도.txt");
    // let file_path = String::from("D:\\workspace\\vscode2\\hello_cargo\\res\\경기도.csv");
    // let file_path = String::from("D:\\res\\csv_sample\\1994.csv");

    let now = Instant::now();
    if let Err(err) = csv_open(&file_path) {
        println!("error running example: {}", err);
        process::exit(1);
    }
    println!("{}", now.elapsed().as_secs_f64());

}

