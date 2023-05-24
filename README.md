# boardlogger

BoardLogger is a lightweight tool designed to monitor and log keyboard inputs and clipboard paste operations on MacOS and Ubuntu systems. The log data is written to a directory specified by the user when running the program. BoardLogger is built with performance in mind, and operates with minimal memory footprint to avoid affecting system performance.

## Installation

1. Clone the repository to your local machine.

```bash
git clone https://github.com/semigrp/boardlogger.git
```

2. Navigate to the cloned repository
    
```bash
cd boardlogger
```

3. Build the project with cargo

```bash
cargo build --release
```

The executable file will be generated at target/release/boardlogger.

Usage
-----
To start logging keyboard and clipboard events, run the generated executable file with the path of the log directory as the argument

```bash
./target/release/boardlogger /path/to/log/directory/your.md
```
This will create a new log file in the specified directory, with the current date as the file name (for example, 20230524.log). The program will append to this file until it is terminated.

The log file records the timestamped keyboard inputs and clipboard contents each time a change is detected. Keyboard inputs are logged as a list of keys, and clipboard contents are logged as a string.

### Contribution
If you'd like to contribute to this project, please feel free to create a pull request or open an issue on this repository.

### License
This project is licensed under the MIT License. Please see the LICENSE file for more details.

### Disclaimer
Use BoardLogger responsibly. It's not intended to be used as a tool for unauthorized surveillance or privacy invasion.
