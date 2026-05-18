use syn::{DeriveInput, Field, Type, Data};

use crate::database_config::get_mapper; 
use crate::TypeMapper;


/// Field Processor, including the information of the all the fields in a struct 
#[derive(Clone)]
pub struct FieldProcessor {
    //all the fields info in the struct
    fields: Vec<FieldInfo>,
    //the name of the struct
    struct_ident: String,
    //the name of the primary key field, if we have not set it, it will create a primary key.
    primary_key: Option<(String, Type)>,    
    //ignore the fields that are not used in the table or sql statement
    ignore_fields_name: Vec<String>,
}



///The field information in a struct
#[derive(Clone)]
pub struct FieldInfo {
    pub name: String,
    pub ty: Type,
    pub is_primary_key: bool,
    pub is_ignore: bool,
}

impl FieldInfo {
    // Create a new FieldInfo derectly from Field
    fn new(name: String, ty: Type, is_primary_key: bool, is_ignore: bool) -> Self {
        Self {
            name,
            ty,
            is_primary_key,
            is_ignore,
        }
    }
}

///
impl FieldProcessor {
    /// Create a new FieldProcessor derectly from DeriveInput
    /// # Note: Only struct can be used with this function
    pub fn new(input: &DeriveInput) -> Self {
        let struct_ident = input.ident.to_string();
        let mut  primary_key: Option<(String, Type)> = None;
        let mut ignore_fields_name: Vec<String> = Vec::new();
        let fields = match & input.data {
            Data::Struct(s) => {
                let mut _field_name: String = String::new();
                let mut is_ignore = false;
                let mut is_primary_key = false;
                

                //iter the fields of the struct
                s.fields.iter().map(
                    |f:&Field | {
                        //reset the flags
                        is_ignore = false;
                        is_primary_key = false;

                        _field_name = f.ident.as_ref().unwrap().to_string();
                        let field_ty = f.ty.clone();

                        //iter the attributes of the field
                        f.attrs.iter().for_each(
                            //check if the field is primary key or need to be ignored in
                            // the table or sql statement
                            |a| {
                                let attr_ident = a.path.get_ident().expect("error in attr of fields in a struct");
                                if attr_ident == "ignore" {
                                    ignore_fields_name.push(_field_name.clone());
                                    is_ignore = true;
                                }else if attr_ident == "primary_key" {
                                    //check if we have already set the primary key field, if it is, that's
                                    //an error, store the primary 
                                    if primary_key.is_none() {
                                        primary_key = Some((_field_name.clone(), field_ty.clone()));
                                        is_primary_key = true;
                                    }else{
                                        //we can't mark two fields as primary key in the same sql table
                                        panic!("Error: multiple primary key fields found in struct {}", struct_ident);
                                    }
                                }

                            }
                        );

                        FieldInfo::new(_field_name.clone(), field_ty, is_primary_key, is_ignore)
                    }
                
                //collect the fields info in the struct    
                ).collect::<Vec<FieldInfo>>()
            
            },


            _=> panic!("Error: only struct can be used to create a table"),
        };

         Self {
            fields,
            struct_ident,
            primary_key,
            ignore_fields_name,
        }
        
    }

    //get all feilds info in the struct
    pub fn get_fields(&self) -> &Vec<FieldInfo> {
        &self.fields
    }

    //get the name of the struct, we will convert it into all lower case
    pub fn get_struct_ident(&self) -> String{
        self.struct_ident.to_lowercase()
    }

    //get the name of the primary key field
    pub fn get_primary_key(&self) -> Option<(String, Type)> {
        self.primary_key.clone()
    }

    pub fn get_ignore_fields_name(&self) -> &Vec<String> {
        &self.ignore_fields_name
    }

    ///This function will return a string of the column list of the table.
    /// # Note:
    /// It will filter out the fields that are marked as ignore or primary key,
    /// BOTH.
    pub fn get_column_list_without_primary_key(&self) -> Vec<String> {
        self.fields.iter().filter(|f| 
            !f.is_ignore && !f.is_primary_key
        ).map(|f|  f.name.clone() ).collect::<Vec<String>>()
    }

    ///This function will return a string of the column list of the table.
    /// # Note:
    /// It will filter out the fields that are marked as ignore.
    /// The primary key field will be remained in this vector.
    pub fn get_column_list(&self) -> Vec<String> {
        self.fields.iter().filter(|f| 
            !f.is_ignore
        ).map(|f|  f.name.clone() ).collect::<Vec<String>>()
    }

    ///mapper all the rust type into sql type String, such as  rust type i32 will be mapped
    /// to string "INTEGER".
    /// # Note:
    /// It will filter out the fields that are marked as ignore. 
    /// The primary key field will be REMAINED in this vector.
    pub fn get_sql_type_list(&self) -> Vec<String> {
        //get the mapper to map the rust type
        let mapper = get_mapper();

        //get all the fields Type struct in the FieldProcessor
        let vec_field_info = self.get_fields();
        let ret = vec_field_info.iter()
            .filter(|f| !f.is_ignore )
            .map(|f| {
            mapper.map_type(&f.ty).to_string()
        }).collect::<Vec<String>>();
 
        ret
    }

    ///mapper all the rust type into sql type String, such as  rust type i32 will be mapped
    /// to string "INTEGER".
    /// # Note:
    /// It will filter out the fields that are marked as ignore. 
    /// The primary key field will be FILTERED out in this vector.

    pub fn get_sql_type_list_without_primary_key(&self) -> Vec<String> {
        
        //get the mapper to map the rust type
        let mapper = get_mapper();

        //get all the fields Type struct in the FieldProcessor
        let vec_field_info = self.get_fields();
        let ret = vec_field_info.iter()
            .filter(|f| !f.is_ignore )
            .map(|f| {
            mapper.map_type(&f.ty).to_string()
        }).collect::<Vec<String>>();
 
        ret
    
    }


}



#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    ///a function to create a struct, and parse it into a DeriveInput
    /// # Note:
    /// This function should not be changed for the test structure we create inside 
    /// will INFLUENCE the test result of the other tests DIRECTLY.
    #[inline(always)]
    fn gen_test_token_stream() -> DeriveInput {
        parse_quote! {
            struct TestStruct {
                #[primary_key]
                id: i32,
                name: String,
                #[ignore]
                age: i32,
            }
        }
    }

    #[test]
    fn test_field_processor() {
        let input = gen_test_token_stream();

        let field_processor = FieldProcessor::new(&input);
        let fields = field_processor.get_fields();
        let struct_ident = field_processor.get_struct_ident();
        let primary_key = field_processor.get_primary_key();
        let ignore_fields_name = field_processor.get_ignore_fields_name();

        assert_eq!(fields.len(), 3);

        //the ident will be converted to lower case
        assert_eq!(struct_ident, "test_struct");    

        assert_eq!(primary_key.unwrap().0, "id".to_string());
        assert_eq!(ignore_fields_name.len(), 2);
        assert_eq!(ignore_fields_name[0], "name");
        assert_eq!(ignore_fields_name[1], "age");

    }

    #[test]
    fn test_get_sql_list() {
        let input = gen_test_token_stream();

        let field_processor = FieldProcessor::new(&input);
        let sql_type_list = field_processor.get_sql_type_list();

        assert_eq!(sql_type_list.len(), 1);
        println!("sql_type_list: {:?}", sql_type_list);
       
    }

    #[test]
    fn test_get_column_list_without_primary_key() {
        let input = gen_test_token_stream();
        let field_processor = FieldProcessor::new(&input);
        println!("{:?}", field_processor.get_column_list_without_primary_key());
        assert_eq!(field_processor.get_column_list_without_primary_key().len(), 1);
    }

    #[test]
    fn test_get_column_list() {
        let input = gen_test_token_stream();
        let field_processor = FieldProcessor::new(&input);
        println!("{:?}", field_processor.get_column_list());
        assert_eq!(field_processor.get_column_list().len(), 2);
    }

}