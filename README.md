Text Search in Files
=====================

This command-line program searches for a given text in a file and prints the line that contains the text. It also allows the user to specify whether the search should be case-sensitive or case-insensitive. The program handles errors properly and provides user-friendly messages.

Installation
--------------

To use this program, you need to have Rust installed on your system. You can install Rust from the official Rust installation page.

Building and Running
------------------------

1. Clone the repository:
   ```bash
   git clone https://github.com/swataswayam-14/TextTrove

2. Navigate to the project directory:
   ```bash
   cd text-trove

3. Build the program:
   ```bash
   cargo build

4. Run the program: 
   ```bash
   cargo run -- <query_string> <file_name>.txt


Usage
------

To use the program, you need to provide the path to the file you want to search and the text you want to search for. You can also specify whether the search should be case-sensitive or case-insensitive by setting the IGNORE_CASE environment variable.

1. Run the program: 
   ```rust
   IGNORE_CASE=1 cargo run -- <query_string> <file_name>.txt


Testing
--------

The program has been thoroughly tested using Rust’s built-in testing framework. You can run the tests using the following command:

1. Run tests:
   ```bash
   cargo test


Contributing
-------------

Contributions are welcome. If you find any issues or have suggestions for improvement, please open an issue or submit a pull request.
