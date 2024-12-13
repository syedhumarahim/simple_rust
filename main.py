import sys
from lib.crud import extract, load, query as query_fn

def main():
    # Step 1: Get command line arguments
    args = sys.argv
    if len(args) < 2:
        print(f"Usage: {args[0]} [action]")
        return

    action = args[1]

    if action == "extract":
        # Extract
        url = "https://github.com/syedhumarahim/syedhumarahim-dataset_medical_records/blob/main/medical_records.csv"
        file_path = "data/medical_records.csv"
        directory = "data"

        print("Extracting data...")
        extract(url, file_path, directory)
        print("Data extraction completed.")

    elif action == "load":
        # Load
        file_path = "data/medical_records.csv"
        print("Loading data into MedicalRecordsDB...")
        try:
            db_file = load(file_path)
            print(f"Data successfully loaded into {db_file}.")
        except Exception as e:
            print(f"Failed to load data: {e}")

    elif action == "query":
        if len(args) > 2:
            q = args[2]
            try:
                query_fn(q)
                print("Query executed successfully!")
            except Exception as err:
                print(f"Error: {err}")
        else:
            print(f"Usage: {args[0]} query [SQL query]")

    else:
        print("Invalid action. Use 'extract', 'load', or 'query'.")

if __name__ == "__main__":
    main()
