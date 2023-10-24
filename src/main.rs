use sqlite_database::*;  // this is crate's library, which is now in lib.rs
use std::error::Error;
use std::time::Instant;

fn main() {
    match run_program() {
        Ok(1) => println!("Program executed successfully!"),
        Err(e) => eprintln!("Error occurred: {}", e),
        _ => eprintln!("Unknown error occurred."),
    }
}


pub fn run_program() -> Result<i32, Box<dyn Error>> {
    let start = Instant::now();
    let mem_info_before = sys_info::mem_info().unwrap();

    connect_to_database().unwrap();
    create_operation().unwrap();
    read_operation().unwrap();
    update_operation().unwrap();
    delete_operation().unwrap();
    
    let elapsed = start.elapsed();
    let mem_info_after = sys_info::mem_info().unwrap();
    let mem_used = mem_info_after.total - mem_info_before.total;

    std::fs::write(
        "rust_performance.md",
        format!(
            "## Rust Performance Report\n- Time taken: {:.2?} seconds\n- Memory used: {} KB\n### Operations Performed\n1. connect to database\n2. create a table\n3. read a table\n4. updata a table\n5. delete a table\n", 
            elapsed, mem_used
        )
    )?;

    Ok(1)
}


