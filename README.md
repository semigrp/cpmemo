# cpMemo (Clipboard Memo)
cpMemo is a minimalist and efficient tool written in Rust. It is designed to track and log every change made to your clipboard on macOS and Ubuntu systems. By utilizing Rust, cpMemo ensures minimal memory usage, providing a lightweight solution that runs seamlessly in the background.

## Usage

1. Build the project using Cargo: cargo build --release
2. Run the built binary, providing the path to the file where you want to store the logs as an argument:

```bash
./target/release/cpmemo /path/to/your/log/file
```

## Functionality
cpMemo monitors your clipboard and logs each change with a timestamp, saving all the data in a user-specified log file. Every time you copy a piece of text, cpMemo records it. It is an excellent tool for those who want to keep track of the information they copy throughout the day, serving as a useful recovery point for lost clipboard data.

## Stopping cpMemo 
Press **Ctrl+C** to stop cpMemo. The program will automatically save the logs to the specified file.

### License
This project is licensed under the MIT License. Please see the LICENSE file for more details.

### Disclaimer
cpMemo should be used responsibly. It's your responsibility to ensure that you have the necessary permissions to record and store the copied data, especially in a shared or work environment. We are not responsible for any misuse of this tool.

Enjoy your experience with cpMemo!
