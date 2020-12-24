use std::process;
use std::time::Instant;
use datatable::csv_reader;

mod datatable;


fn callback(percent:u32){
    println!("percent : {}%", percent);
}
fn main() {
    // let file_path = "D:\\workspace\\vscode\\lightsheet_rust\\res\\euc_kr.csv".to_string();
    //let file_path = "D:\\res\\csv_sample\\1994.csv".to_string();
    let file_path = "D:\\res\\csv_sample\\전라북도.txt".to_string();
    let db_path = "D:\\workspace\\vscode\\lightsheet_rust\\tmp\\tt.db".to_string();
    let now = Instant::now();
    let ret = csv_reader::read_csv(db_path, file_path, callback);
    if let Err(err) = ret {
        println!("error running example: {}", err);
        process::exit(1);
    }
    let table_info = ret.unwrap();
    let spendtime = now.elapsed().as_secs_f64();
    println!("spendtime : {}", spendtime);
    println!("1s per insert : {}", (table_info.row_len as f64) / spendtime);

}

