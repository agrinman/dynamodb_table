pub use internal::DynamodbTable;
pub use table_macro::DynamodbTable;

#[allow(unused)]
#[derive(DynamodbTable)]
#[dynamodb_table_name("my_table")]
struct TableTest {}

#[allow(unused)]
#[derive(DynamodbTable)]
struct TableTest2 {}

#[allow(unused)]
#[derive(DynamodbTable)]
#[dynamodb_table_name_prefix("prefix_")]
struct TableTest3 {}

#[allow(unused)]
#[derive(DynamodbTable)]
#[dynamodb_table_inherit_from(TableTest3)]
struct TableTest4 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_tests() {
        assert_eq!(TableTest::table_name(), "my_table".to_string());
        assert_eq!(TableTest2::table_name(), "tabletest2".to_string());
        assert_eq!(TableTest3::table_name(), "prefix_tabletest3".to_string());
        assert_eq!(TableTest4::table_name(), TableTest3::table_name());
    }
}
