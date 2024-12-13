use huma::{extract, load, query as query_fn};
use std::env;

fn main() {
    // Step 1: Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];

    match action.as_str() {
        "extract" => {
            // Extract
            let url = "https://github.com/syedhumarahim/syedhumarahim-dataset_medical_records/blob/main/medical_records.csv";
            let file_path = "data/medical_records.csv";
            let directory = "data";

            println!("Extracting data...");
            extract(url, file_path, directory);
            println!("Data extraction completed.");
        }
        "load" => {
            // Load
            let file_path = "data/medical_records.csv";
            println!("Loading data into MedicalRecordsDB...");
            match load(file_path) {
                Ok(db_file) => println!("Data successfully loaded into {}.", db_file),
                Err(e) => eprintln!("Failed to load data: {:?}", e),
            }
        }
        "query" => {
            if let Some(q) = args.get(2) {
                if let Err(err) = query_fn(q) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Query executed successfully!");
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        _ => {
            println!("Invalid action. Use 'extract', 'load', or 'query'.");
        }
    }
}
