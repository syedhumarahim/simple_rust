import os
import requests
import csv
import sqlite3

LOG_FILE = "query_log.md"

def log_query(query, log_file):
    # Append the SQL query to the log file
    with open(log_file, "a", encoding="utf-8") as f:
        f.write(f"```sql\n{query}\n```\n")

def extract(url: str, file_path: str, directory: str):
    # Ensure the directory exists
    if not os.path.exists(directory):
        os.makedirs(directory)

    # Download the file
    response = requests.get(url)
    response.raise_for_status()

    # Save the file
    with open(file_path, "wb") as f:
        f.write(response.content)

    print("Extraction successful!")

def load(dataset: str) -> str:
    conn = sqlite3.connect("MedicalRecordsDB.db")
    c = conn.cursor()

    # Drop table if it exists
    c.execute("DROP TABLE IF EXISTS MedicalRecordsDB")

    # Create table
    c.execute("""
        CREATE TABLE MedicalRecordsDB (
            patient_id INTEGER,
            name TEXT,
            date_of_birth TEXT,
            gender TEXT,
            medical_conditions TEXT,
            medications TEXT,
            allergies TEXT,
            last_appointment_date TEXT
        )
    """)

    # Read CSV and insert records
    with open(dataset, newline='', encoding='utf-8') as csvfile:
        rdr = csv.reader(csvfile)

        for record in rdr:
            # Each record is a list of fields
            c.execute("""
                INSERT INTO MedicalRecordsDB (
                    patient_id, name, date_of_birth, gender, medical_conditions, 
                    medications, allergies, last_appointment_date
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            """, record)

    conn.commit()
    conn.close()

    return "MedicalRecordsDB.db"

def query(q: str):
    conn = sqlite3.connect("MedicalRecordsDB.db")
    c = conn.cursor()

  
    q_lower = q.strip().lower()
    if q_lower.startswith("select"):
        # Fetch and print results
        c.execute(q)
        rows = c.fetchall()
        for row in rows:
            (patient_id, name, date_of_birth, gender, medical_conditions,
             medications, allergies, last_appointment_date) = row
            print(f"Result: id={patient_id}, name={name}, dob={date_of_birth}, gender={gender}, "
                  f"conditions={medical_conditions}, meds={medications}, allergies={allergies}, "
                  f"last_appointment={last_appointment_date}")
    else:
        # For INSERT/UPDATE/DELETE
        c.executescript(q)
        conn.commit()

    # Log the query
    log_query(q, LOG_FILE)

    conn.close()
