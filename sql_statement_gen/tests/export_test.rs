use sql_statement_gen::SqlStatement;


struct MyStruct {
    pub field1: i32,
    field2: String,
}

#[test]
fn exprot_text() {
    let test = MyStruct {
        field1: 1,
        field2: "test".to_string(),
    };
    let sql = test.create_table_sql();
    println!("SQL: {}", sql);   
}
