
pub fn colname(col_count:u32) -> String {
    let mut owned_string: String = "c_".to_owned();
    let s: String = col_count.to_string();
    owned_string.push_str(&s);
    owned_string
}
