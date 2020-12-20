use super::csv_reader;
use std::time::Instant;
use std::str;

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
    if let Err(err) = csv_reader::csv_open(&file_path) {
        println!("error running example: {}", err);
        assert_eq!(true, false);
    }
    println!("{}", now.elapsed().as_secs_f64());
}
