/// Define a table name for this structure
pub trait DynamodbTable {
    /// the name of the table in dynamodb
    fn table_name() -> String;
}
