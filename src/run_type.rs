//declare a struct, -> string name, enum type and order 
#[derive(Clone)]
pub enum SqlType {
    Table
}

pub struct SqlRunType {
    pub sql_type: SqlType,
    pub folder_name: String,
    pub order: i8,
}

pub fn get_types() -> Vec<SqlRunType> {
    return vec![
        SqlRunType{sql_type: SqlType::Table, folder_name: "table".to_string(), order: 5}
    ];
}