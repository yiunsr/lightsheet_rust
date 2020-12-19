use super::db_utils;

#[test]
fn test_colname(){
    let colname = db_utils::colname(10);
    assert_eq!("c_10", colname);
}