use std::process;
use std::time::Instant;
use datatable::csv_reader;

mod datatable;


fn callback(percent:u32){
    println!("percent : {}%", percent);
}
fn main() {
    let file_path = "D:\\workspace\\vscode\\lightsheet_rust\\res\\euc_kr.csv".to_string();
    let db_path = "D:\\workspace\\vscode\\lightsheet_rust\\tmp\\tt.db".to_string();
    let now = Instant::now();
    let ret = csv_reader::read_csv(db_path, file_path, callback);
    if let Err(err) = ret {
        println!("error running example: {}", err);
        process::exit(1);
    }
    println!("{}", now.elapsed().as_secs_f64());

}

