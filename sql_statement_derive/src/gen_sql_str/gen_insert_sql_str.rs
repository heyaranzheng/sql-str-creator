///Thi function was used to generate the insert sql string for the given table and columns.
use super::*;

use crate::database_config::DATABASE_PALCE_HOLDER;




pub  fn gen_insert_sql_statement(
    table_name: String,
    primary_key: (String, String),
    column_list: &Vec<(String, String)>,
    user_list_index: Option<&Vec<usize>>,
) -> String {
    //we create a chain list, it's a iterator. We  can iterate over it by for loop.
    let list = 
        std::iter::once(&primary_key).chain(column_list.iter());
    
    //all the insert field names
    let mut insert_field_names: Vec<&str> = Vec::new();

    //if we have a user  list, we need to filter the field names.
    if let Some(indexs) = user_list_index {
        let mut  counter: usize = 0;
        for (name, _) in list {
            if counter == indexs[counter] {
                insert_field_names.push(&name);
            } 
            counter += 1;
        }
    }else {
        //we don't have a user list, we need to get all the field names. We use all the column list.
        insert_field_names = column_list.iter().map(|(name, _)| name.as_str()).collect();
    }
    
    //ret: "INSERT INTO"
    let mut ret = String::from("INSERT INTO ");

    //ret: "INSERT INTO 'table_name'"
    ret.push_str(&table_name);

    //add a '(' and the primary key at the first palce.
    //ret may like this if you set id as primary key for example:
    // "INSERT INTO 'table_name' ( id, name, age) " 
    let mut str =  format!("( {} )", gen_field_name_str(&insert_field_names));
    ret.push_str(&str);

    

    //add the value list
    //ret may be like this : "INSERT INTO 'table_name' (id, name, age) VALUES (?1, ?2, ?3)" for
    //sqlite.
    str = format!(" VALUES ({})", gen_value_list(&insert_field_names));
    ret.push_str(&str);

    
    ret
}

/// function to generate string like " id, name, age" with a given column list.
/// if the you have set primary_key on id for example, the string will be like 
/// " name, age" 
/// # Note
///  this will IGNORE the primary key field and the field marked with ignore.
fn gen_field_name_str(list: &Vec<&str>) -> String {
    let mut ret = String::new();
    let len = list.len();
    let mut  _temp = String::new();
    for i in 0 .. (len - 1) {
        _temp = format!(" {}, ", list[i]);
        ret.push_str(&_temp);
    }

    //leave the last field name without comma
    ret.push_str(&list[len - 1]);

    ret
}

/// function to generate string like " ?1, ?2, ?3" with a given column list, if you set
/// feature "only-for-sqlite".
/// For postgres, it will be like " $1, $2, $3"
fn gen_value_list(list: &Vec<&str>) -> String {
    let mut ret = String::new();
    let len = list.len();
    let mut _temp = String::new();

    //we need to remain a placeholder for the primary key field. So we need to add 1 to the
    //length of the column list.
    for i in 0 .. len  {
        _temp = format!(" {}{}, ",DATABASE_PALCE_HOLDER, i + 1);
        ret.push_str(&_temp);
    }

    //leave the last field name without comma
    ret.push_str(&format!(" {}{}", DATABASE_PALCE_HOLDER, len + 1));

    ret
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::field_processor::FieldProcessor;
    use syn::parse_quote;
  
    /// parse the test struct, and return some objects for test.
    /// # Returns
    /// * `Field_processor` - the FieldProcessor object for the test struct.
    /// * 'Vec<(String, String)>` - the column list of the test struct.
    /// * `table_name` - the table name of the test struct.
    /// * `primary_key` - the primary key of the test struct.
    fn gen_test_token_stream() 
    -> (FieldProcessor,
        Vec<(String, String)>, 
        String, 
        (String, String)
    )
        
    {
        let input = parse_quote! {
            struct TestStruct {
                #[primary_key]
                id: i32,
                #[ignore]
                name: String,
                #[ignore]
                age: i32,
                height: f32,
                weight: f32,
                #[ignore]
                score: u32,
            }
        };

        let field_processor = FieldProcessor::new(&input);   
        let column_list = field_processor.get_column_list();
        let table_name = field_processor.get_table_name();
        let primary_key = field_processor.get_primary_key();

        (field_processor, column_list, table_name, primary_key)
    }
    
    #[test]
    fn test_gen_field_name_str() {
        let (_, column_list, _, _) = gen_test_token_stream();
        let  list = column_list.iter().map(|(name, _)| name.as_str()).collect();
        let field_name_str = gen_field_name_str(&list);
        assert_eq!(field_name_str, " height, weight");

    }

    #[test]
    fn test_gen_insert_sql_str() {
        let (field_processor, column_list, 
            table_name, primary_key
        ) = gen_test_token_stream();

        let str = gen_insert_sql_statement(
            table_name,
            primary_key,
            &column_list,
            None
        );
        println!("{}", str);
    }
}