pub trait Database {
    // Tables
    fn create_table(name: str) -> ();
    fn drop_table(table_id: i8) -> ();

    // CRUD
    fn insert(table_id: i8, value: i8) -> ();
    fn find(table_id: i8, key: i8) -> ();
    fn update(table_id: i8, key: i8, value: i8) -> ();
    fn delete(table_id: i8, key: i8) -> ();

    // Etc
    fn print_index() -> ();
    fn print_leaves() -> ();
    fn print_tables() -> ();
}
