use super::db_utils;

#[test]
fn test_colname(){
    let colname = db_utils::colname(10);
    assert_eq!("c_10", colname);
}

#[test]
fn test_create_query_main(){
    let table_name = String::from("datatable");
    let extected_query = "CREATE TABLE `datatable` (`id` INTEGER PRIMARY KEY, `c_0` string NOT NULL DEFAULT \'\' , `c_1` string NOT NULL DEFAULT \'\' );";
    let query = db_utils::create_query_main(1u32, 2, false);
    assert_eq!(extected_query, query);
}

#[test]
fn test_insert_query(){
    let table_name = String::from("datatable");
    let extected_query = "INSERT INTO `datatable`(id, c_0 , c_1)  values (?,?,?)";
    let query = db_utils::insert_query(1u32, 2);
    assert_eq!(extected_query, query);
}

#[test]
fn test_select_query(){
    let table_name = String::from("datatable");
    let extected_query = r#"Select id, c_0 , c_1 From datatable"#;
    let query = db_utils::select_query(1u32, 2,
        String::from(""), String::from(""), String::from(""));
    assert_eq!(extected_query, query);
}

#[test]
fn test_distinct_col_query(){
    let table_name = String::from("datatable");
    let query = db_utils::distinct_col_query(&table_name, 2);
    let extected_query = "Select distinct c_2 From datatable";
    assert_eq!(extected_query, query);   
}

#[test]
fn test_select_col_query(){
    let table_name = String::from("datatable");
    let query = db_utils::select_col_query(&table_name, 2);
    let extected_query = "Select c_2 From datatable";
    assert_eq!(extected_query, query);   
}
