use log;
use std::fmt::format;

#[repr(u16)]
pub enum TableType{
    MainTable, 
    TableMeta,
    RowMeta,
    ColMeta,
}

pub fn colname(col_count:u32) -> String {
    let mut owned_string: String = "c_".to_owned();
    let s: String = col_count.to_string();
    owned_string.push_str(&s);
    owned_string
}

pub fn create_query_main(window_id:u32, col_count: u32, has_row_id: bool) -> String {
    let tablename = get_table_name(window_id, TableType::MainTable);
    let mut col_names_query = String::new();
    for i in 0..col_count-1 {
        col_names_query.push('`');
        col_names_query.push_str(&colname(i));
        col_names_query.push_str(&String::from("` TEXT NOT NULL DEFAULT '' , ")
    );
    }
    col_names_query.push('`');
    col_names_query.push_str( &colname(col_count-1));
    col_names_query.push_str( &"` TEXT NOT NULL DEFAULT '' ");
    let mut query = String::from("CREATE TABLE `");
    query.push_str(&tablename);
    query.push_str("` (`id` INTEGER PRIMARY KEY, ");
    query.push_str(&col_names_query);
    if has_row_id{
        query.push_str(") WITHOUT ROWID;");
    }
    else{
        query.push_str(");");
    }
    log::info!("create_query_main : {}", query);
	query
}

pub fn create_query_rowmeta(window_id:u32) -> String {
    let tablename = get_table_name(window_id, TableType::RowMeta);
    let mut query = String::from("CREATE TABLE `");
    query.push_str(&tablename);
    query.push_str("`( row_meta_id INTEGER NOT NULL,");
	query.push_str("row_idx INTEGER NOT NULL,");
    query.push_str("row_meta_status INTEGER NOT NULL,");
    query.push_str("PRIMARY KEY('row_meta_id')");
    query.push_str(") WITHOUT ROWID;");
    log::info!("create_query_rowmeta : {}", query);
    query
}

pub fn drop_query(window_id:u32, table_type:TableType) -> String {
    let tablename = get_table_name(window_id, table_type);
    let mut query = String::from("Drop TABLE `");
    query.push_str(&tablename);
    query.push_str("`;");
    log::info!("drop_query : {}", query);
	query
}

pub fn insert_query(window_id:u32, col_count: u32) -> String {
    let tablename = get_table_name(window_id, TableType::MainTable);
    let mut query = String::from("INSERT INTO `");
    query.push_str(&tablename);
    query.push('`');
    let mut col_names_query = String::from("id, ");
    let mut col_values_query = String::from("?,");
    for i in 0..col_count-1 {
        col_names_query.push_str(&colname(i));
        col_names_query.push_str(" , ");
        col_values_query.push_str( "?,");
    }
    col_names_query .push_str(&colname(col_count - 1));
    col_values_query.push_str("?");
    query.push('(');
    query.push_str(&col_names_query);
    query.push_str(")  values (");
    query.push_str(&col_values_query);
    query.push(')');
    log::info!("insert_query : {}", query);
	query
}

pub fn insert_query_rowmeta(window_id:u32) -> String {
    let tablename = get_table_name(window_id, TableType::RowMeta);
    let tablename_main = get_table_name(window_id, TableType::MainTable);
    let mut query = String::from("INSERT INTO `");
    query.push_str(&tablename);
    query.push_str("`(row_meta_id, row_idx, row_meta_status)");
    query.push_str(" SELECT id, id, 1 FROM ");
    query.push_str(&tablename_main);
    query.push(';');
    query
}

pub fn select_query(window_id:u32, col_count:u32, where_:String, group:String, having: String) -> String {
    let tablename = get_table_name(window_id, TableType::MainTable);
    let tablename_rowmeta = get_table_name(window_id, TableType::RowMeta);
    let mut query = String::from("Select row_idx,");
    let mut col_names_query = String::new();
    for i in 0..col_count-1 {
        col_names_query.push_str(&colname(i));
        col_names_query.push_str(" , ");
    }
	col_names_query.push_str(&colname(col_count-1));
    query.push_str(&col_names_query);
    query.push_str(" From ");
    query.push_str(&tablename);
    let join_q = format!(" LEFT JOIN {rowmeta} ON ({main}.id = {rowmeta}.row_meta_id AND {rowmeta}.row_meta_status=1) ", 
        main=tablename, rowmeta=tablename_rowmeta);
    query.push_str(&join_q);
    if where_.len() > 0 {
        query.push_str(" Where ");
        query.push_str(&where_);
    }
	if group.len() > 0 {
        query.push_str(" Group BY ");
        query.push_str(&group);
    }
    if having.len() > 0 {
        query.push_str(" Having ");
        query.push_str(&having);
    }
    let order_q = format!(" ORDER BY {rowmeta}.row_idx;", rowmeta=tablename_rowmeta);
    query.push_str(&order_q);
	log::info!("select_query : {}", query);
	query
}

pub fn move_for_add_rows_query_rowmeta(window_id:u32, row_idx: u32, row_add_count: u32) -> String{
    let mut query = String::from("UPDATE ");
    let tablename = get_table_name(window_id, TableType::RowMeta);
    query.push_str(&tablename);
    query.push_str(" SET row_idx = row_idx +");
    query.push_str(&row_add_count.to_string());
    query.push_str(" WHERE row_idx >= ");
    query.push_str(&row_idx.to_string());
    query.push_str(";");
    query
}

pub fn insert_blank_query(window_id:u32, col_len: u32) -> String{
    let tablename = get_table_name(window_id, TableType::MainTable);
    let mut query = String::from("INSERT INTO `");
    query.push_str(&tablename);
    query.push('`');
    let mut col_names_query = String::from("id, ");
    let mut col_values_query = String::from("null,");
    for i in 0..col_len-1 {
        col_names_query.push_str(&colname(i));
        col_names_query.push_str(" , ");
        col_values_query.push_str( "'',");
    }
    col_names_query .push_str(&colname(col_len - 1));
    col_values_query.push_str("''");
    query.push('(');
    query.push_str(&col_names_query);
    query.push_str(")  values (");
    query.push_str(&col_values_query);
    query.push(')');
    log::info!("insert_query : {}", query);
	query
}

pub fn insert_blank_query_rowmeta(window_id:u32) -> String{
    let tablename = get_table_name(window_id, TableType::RowMeta);
    let mut query = String::from("INSERT INTO ");
    query.push_str(&tablename);
    query.push_str(&"(row_meta_id, row_idx, row_meta_status)  VALUES(?, ?, 1);");
    query
}

pub fn distinct_col_query(tablename:&String, col_index: u32) -> String {
    let mut query = String::from("Select distinct ");
    query.push_str(&colname(col_index));
    query.push_str(" From ");
    query.push_str(&tablename);
	return query
}

pub fn select_col_query(tablename:&String, col_index:u32) -> String {
    let mut query = "Select ".to_string();
    query.push_str(&colname(col_index));
    query.push_str(" From ");
    query.push_str(&tablename);
	return query
}

pub fn get_table_name(window_id:u32, table_type:TableType) -> String{
    let mut table_name = "datatbl_".to_string();
    match table_type{
        TableType::MainTable=>{}
        TableType::TableMeta=>{table_name.push_str(&"meta_".to_string());}
        TableType::RowMeta=>{table_name.push_str(&"row_meta_".to_string());}
        TableType::ColMeta=>{table_name.push_str(&"col_meta_".to_string());}
    }
    table_name.push_str(&window_id.to_string());
    table_name
}
