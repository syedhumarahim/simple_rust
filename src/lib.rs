use reqwest::blocking::Client;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const LOG_FILE: &str = "query_log.md";

fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

pub fn extract(url: &str, file_path: &str, directory: &str) {
    if !fs::metadata(directory).is_ok() {
        fs::create_dir_all(directory).expect("Failed to create directory");
    }

    let client = Client::new();
    let mut response = client.get(url).send().expect("Failed to send request");
    let mut file = fs::File::create(file_path).expect("Failed to create file");

    std::io::copy(&mut response, &mut file).expect("Failed to copy content");

    println!("Extraction successful!");
}

pub fn load(dataset: &str) -> Result<String> {
    let conn = Connection::open("MedicalRecordsDB.db")?;

    conn.execute("DROP TABLE IF EXISTS MedicalRecordsDB", [])?;

    conn.execute(
        "CREATE TABLE MedicalRecordsDB (
            patient_id INTEGER,
            name TEXT,
            date_of_birth TEXT,
            gender TEXT,
            medical_conditions TEXT,
            medications TEXT,
            allergies TEXT,
            last_appointment_date TEXT
        )",
        [],
    )?;

    let mut rdr = csv::Reader::from_path(dataset).expect("Failed to read dataset");

    let mut stmt = conn.prepare(
        "INSERT INTO MedicalRecordsDB (
            patient_id,
            name,
            date_of_birth,
            gender,
            medical_conditions,
            medications,
            allergies,
            last_appointment_date
        ) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                stmt.execute(&[
                    &record[0], // patient_id
                    &record[1], // name
                    &record[2], // date_of_birth
                    &record[3], // gender
                    &record[4], // medical_conditions
                    &record[5], // medications
                    &record[6], // allergies
                    &record[7], // last_appointment_date
                ])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }

    Ok("MedicalRecordsDB.db".to_string())
}

pub fn query(query: &str) -> Result<()> {
    let conn = Connection::open("MedicalRecordsDB.db")?;
    // Read operation
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i32>(0)?,    // patient_id
                row.get::<usize, String>(1)?, // name
                row.get::<usize, String>(2)?, // date_of_birth
                row.get::<usize, String>(3)?, // gender
                row.get::<usize, String>(4)?, // medical_conditions
                row.get::<usize, String>(5)?, // medications
                row.get::<usize, String>(6)?, // allergies
                row.get::<usize, String>(7)?, // last_appointment_date
            ))
        })?;

        for result in results {
            match result {
                Ok((id, name, dob, gender, conditions, meds, allergies, last_appointment)) => {
                    println!(
                        "Result: id={}, name={}, dob={}, gender={}, conditions={}, meds={}, allergies={}, last_appointment={}",
                        id, name, dob, gender, conditions, meds, allergies, last_appointment
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        // Other CUD operations
        let _num_affected_rows = conn.execute_batch(query)?;
    }
    log_query(query, LOG_FILE);
    Ok(())
}
