use sqlite_database::*;  // this is your crate's library, which is now in lib.rs

fn main() {
    connect_to_database().unwrap();
    create_operation().unwrap();
    read_operation().unwrap();
    update_operation().unwrap();
    delete_operation().unwrap();
}
