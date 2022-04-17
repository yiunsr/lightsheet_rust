use super::csv_reader;
use std::time::Instant;
use std::fs::File;

#[test]
fn test_get_col_sep(){
    let csv_str1 = "a1,b1,c1,d1\na2,b2,c2,d2".to_string();
    let sep1 = csv_reader::get_col_sep(&csv_str1);
    assert_eq!(b',', sep1);

    let csv_str2 = "a1\tb1\tc1\td1\na2\tb2\tc2\td2".to_string();
    let sep2 = csv_reader::get_col_sep(&csv_str2);
    assert_eq!(b'\t', sep2);

    let csv_str3 = "a1|b1|c1|d1\na2|b2|c2|d2".to_string();
    let sep3 = csv_reader::get_col_sep(&csv_str3);
    assert_eq!(b'|', sep3);
}


#[test]
fn test_csv_open(){
    let file_path = String::from("D:\\workspace\\vscode2\\hello_cargo\\res\\경상남도.txt");
    // let file_path = String::from("D:\\workspace\\vscode2\\hello_cargo\\res\\경기도.csv");
    // let file_path = String::from("D:\\res\\csv_sample\\1994.csv");
    // let file_path = String::from(".\\res\\auction_202011.csv");
    let now = Instant::now();
    let mut file= &mut File::open(file_path).unwrap();
    let result_open = csv_reader::csv_open(file);
    let _result_open = match result_open{
        Ok(r) => r,
        Err(err) =>{
            panic!("Problem csv_open: {:?}", err)
        }
    };
    println!("{}", now.elapsed().as_secs_f64());
}


fn test_callback(percent:u32){
    println!("percent : {}%", percent);
}

#[test]
fn test_read_csv(){
    println!("한국어 테스트");
    let file_path = "D:\\workspace\\vscode2\\hello_cargo\\res\\경상남도.txt".to_string();
    let db_path = "D:\\workspace\\vscode\\lightsheet_rust\\tmp\\tt.db".to_string();
    csv_reader::read_csv(db_path, file_path, 1u32, test_callback);
}

#[test]
fn test_read_csv_01(){
    let file_path = "D:\\workspace\\vscode\\lightsheet_rust\\res\\euc_kr.csv".to_string();
    let db_path = "D:\\workspace\\vscode\\lightsheet_rust\\tmp\\tt.db".to_string();
    let _ret = csv_reader::read_csv(db_path, file_path, 1u32, test_callback);
    // match ret {
    //     Ok(file) => file,
    //     Err(err) => {
    //         println!("Err:  {:?}", err);
    //         panic!("Problem read_csv: {:?}", err);
    //     },
    // };
}


