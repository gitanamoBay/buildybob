//declare a struct, -> string name, enum type and order
#[derive(Clone)]
pub enum SqlType {
    PreDeployment,
    TableSpace,
    Schema,
    Table,
    Sproc,
    PostDeployment,
}

pub struct SqlRunType {
    pub sql_type: SqlType,
    pub folder_name: String,
    pub drop_procedure: String,
}

pub fn get_types() -> Vec<SqlRunType> {
    return vec![SqlRunType {
                    sql_type: SqlType::PreDeployment,
                    folder_name: "pre-deployment".to_string(),
                    drop_procedure: "".to_string(),
                },
                SqlRunType {
                    sql_type: SqlType::TableSpace,
                    folder_name: "table-space".to_string(),
                    drop_procedure: "DROP TABLESPACE ".to_string(),
                },
                SqlRunType {
                    sql_type: SqlType::Schema,
                    folder_name: "schema".to_string(),
                    drop_procedure: "DROP SCHEMA ".to_string(),
                },
                SqlRunType {
                    sql_type: SqlType::Table,
                    folder_name: "table".to_string(),
                    drop_procedure: "DROP TABLE ".to_string(),
                },
                SqlRunType {
                    sql_type: SqlType::Sproc,
                    folder_name: "sproc".to_string(),
                    drop_procedure: "DROP FUNCTION ".to_string(),
                },
                SqlRunType {
                    sql_type: SqlType::PostDeployment,
                    folder_name: "post-deployment".to_string(),
                    drop_procedure: "".to_string(),
                }];
}
