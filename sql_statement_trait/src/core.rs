use crate::error::Error;


pub trait SqlStatement {
    /// Return the sql string of the create table statement for the given structure.
    fn create_table_sql(&self) -> &'static str;
    /// Return the field names of the structure, the primary key field always at the first position.
    fn get_field_names(&self) -> &Vec<&'static str>;
    /// Return the insert statement sql string.
    fn insert_sql_statement(&self, user_list: Vec<&'static str>) -> Result<&'static str, Error>;
    /// Return a delete statement sql string.
    fn delete_sql_statement(&self, where_clause: &'static str) -> Result<String, Error>;
    

    /// Return the select statement sql string.
    /// # Args
    /// * 'select_columns' - The columns to be selected. If it's None, all columns will be selected.
    /// * 'where_clause' - The where clause. It's a string that contains the condition of the select 
    /// statement. It should be in the format of "field_name = ?".
    fn select_sql_statement(
        &self, 
        select_columns: Option<Vec<&'static str>>, 
        where_clause: &'static str, 
    ) -> Result<&'static str, Error>;

 

    /// Checking user's field list, return the relative position of the field_names list.
    /// 
    /// # Args
    /// * 'user_list' - The user's field list. It's a list of field names. Only the names of the 
    /// structure's fields without marked '#[ignore]' are valid. Otherwise, This function 
    /// will return a InvalidFieldName error.
    /// 
    /// # Returns
    /// * 'Vec<usize>' - The relative position of the field_names list. Every field's name have a fixed 
    /// position in the field_names list. This position is the same as the column index in the insert 
    /// statement.
    ///  * The primary key field always at the first position.
    ///  * The other fields are sorted by the order of appearance in the structure.
    ///  * The fields marked '#[ignore]' are ignored.
    fn get_field_positions(&self, user_list: Vec<&'static str>) -> Result<Vec<usize>, Error>{
        let field_names = self.get_field_names();

        let mut invalid_field_name = Vec::<& str>::new();
  
        let mut vec_field_positions = Vec::<usize>::new();
        for (i, field) in  user_list.iter().enumerate() {
            if field_names.contains(field){
                vec_field_positions.push(i);
            } else {
                invalid_field_name.push(*field);
            }
        }

        if !invalid_field_name.is_empty(){
            return Ok(vec_field_positions);
        }else {
            return Err(
                Error::InvalidFieldName(
                    format!("Invalid field name: {:?}", invalid_field_name)
                )        
            );
        }
    }


}

