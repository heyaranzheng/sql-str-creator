use rusqlite::Params;


struct MyStruct{
    pub field1: i32,
    pub field2: String,
}




trait MyTrait {
    fn add_one(&self) -> i32;
}

impl MyTrait for MyStruct {
    fn add_one(&self) -> i32 {
        self.field1 + 1
    }
}

struct TestStruct {
    pub field1: i32,
}

impl MyTrait for TestStruct {
    fn add_one(&self) -> i32 {
        self.field1 + 1
    }
}

fn fuc(input: bool) -> Box<dyn MyTrait> {
    if input {
        return Box::new(TestStruct { field1: 10 });
    } else {
        return Box::new(MyStruct {
            field1: 20,
            field2: "test".to_string(),
        });
    } 
}

#[test]
fn other_test() {
    let test = fuc(true);
    let result = test.add_one();
    assert_eq!(result, 11);
}