use huma::{extract, load, query};

#[test]
fn test_extract() {
    let url = "https://github.com/syedhumarahim/syedhumarahim-dataset_medical_records/blob/main/medical_records.csv"; // Update with actual URL
    let file_path = "data/medical_records.csv";
    let directory = "data";

    // Test the extract function
    extract(url, file_path, directory);

    // Check if the file has been successfully created
    assert!(
        std::fs::metadata(file_path).is_ok(),
        "Extracted file not found."
    );
}

#[test]
fn test_load() {
    let dataset = "data/medical_records.csv";

    // Test the load function
    let result = load(dataset);

    // Ensure the database file is created successfully
    assert_eq!(
        result.unwrap(),
        "MedicalRecordsDB.db",
        "Database file name mismatch."
    );
}


