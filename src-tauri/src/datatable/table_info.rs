use std::cell::RefCell;
use std::sync::Arc;
use rusqlite::Connection;

pub struct TableInfo{
    pub conn: Arc<Connection>,
	pub table_name: String,
	pub col_len: u32,
	pub row_len: u32,
}