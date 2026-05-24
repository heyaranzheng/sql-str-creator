enum Error{
    InvalidInput(String),
    SqlError(String),      
}


pub trait SqlStatement {
    /// Return the sql string of the create table statement for the given structure.
    fn create_table_sql(&self) -> &'static str;
    /// Return the field names of the structure, the primary key field always at the first position.
    fn get_field_names(&self) -> Vec<&'static str>;
  
}