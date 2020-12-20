use log;

pub fn colname(col_count:u32) -> String {
    let mut owned_string: String = "c_".to_owned();
    let s: String = col_count.to_string();
    owned_string.push_str(&s);
    owned_string
}

pub fn create_query(tablename:String, col_count: u32) -> String {
    let mut col_names_query = String::new();
    for i in 0..col_count-1 {
        col_names_query.push('`');
        col_names_query.push_str(&colname(i));
        col_names_query.push_str(&String::from("` string NOT NULL DEFAULT '' , "));
    }
    col_names_query.push('`');
    col_names_query.push_str( &colname(col_count-1));
    col_names_query.push_str( &"` string NOT NULL DEFAULT '' ");
    let mut query = String::from("CREATE TABLE `");
    query.push_str(&tablename);
    query.push_str("` (`id` INTEGER PRIMARY KEY, ");
    query.push_str(&col_names_query);
    query.push_str(");");
    log::info!("create_query : {}", query);
	query
}

pub fn insert_query(tablename:String, col_count: u32) -> String {
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

pub fn select_query(tablename:String, col_count:u32, where_:String, group:String, having: String) -> String {
    let mut query = String::from("Select id, ");
    let mut col_names_query = String::new();
    for i in 0..col_count-1 {
        col_names_query.push_str(&colname(i));
        col_names_query.push_str(" , ");
    }
	col_names_query.push_str(&colname(col_count-1));
    query.push_str(&col_names_query);
    query.push_str(" From ");
    query.push_str(&tablename);
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
	log::info!("select_query : {}", query);
	query
}

pub fn distinct_col_query(tablename:String, col_index: u32) -> String {
    let mut query = String::from("Select distinct ");
    query.push_str(&colname(col_index));
    query.push_str(" From ");
    query.push_str(&tablename);
	return query
}

pub fn select_col_query(tablename:String, col_index:u32) -> String {
    let mut query = "Select ".to_string();
    query.push_str(&colname(col_index));
    query.push_str(" From ");
    query.push_str(&tablename);
	return query
}
