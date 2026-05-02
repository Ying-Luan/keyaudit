.PHONY: list run-cli-debug run-cli-release tauri-tauri-dev

# list available commands
list:
	@echo "Available commands:"
	@echo "  run-cli-debug - Run the keyaudit CLI application in debug mode"
	@echo "  run-cli-release - Run the keyaudit CLI application in release mode"
	@echo "  run-tauri-dev - Run the Tauri desktop application in development mode"

# run the CLI application in debug mode
run-cli-debug:
	cargo run -p keyaudit-cli

# run the CLI application in release mode
run-cli-release:
	cargo run -p keyaudit-cli --release

# run the Tauri desktop application in development mode
run-tauri-dev:
	cd desktop && npm run tauri dev
