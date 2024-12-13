rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable 

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

all: format lint test run

extract: 
	cargo run extract

load:
	cargo run load
# Read 
read:
	cargo run -- query "SELECT * FROM MatchResultsDB ORDER BY Round DESC LIMIT 5;"
# Delete
delete:
	cargo run -- query "DELETE FROM MatchResultsDB WHERE Round = 10;"  
# Insert
insert:
	cargo run -- query "INSERT INTO MatchResultsDB (Round, Date, \"Team 1\", \"Team 2\", FT) VALUES (101, '2024-10-24', 'Team A', 'Team B', '2-1');"
# Update
update:
	cargo run -- query "UPDATE MatchResultsDB SET \"Team 1\" = 'MIDS' WHERE \"Team 1\" = 'Team A';"