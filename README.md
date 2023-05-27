String Shuffler

This is a command-line tool written in Rust that allows you to randomly shuffle a given string.
Prerequisites

To use this tool, you need to have Rust and Cargo installed on your system. If you don't have Rust installed, you can get it from the official Rust website: https://www.rust-lang.org/tools/install
Usage

    Clone or download this repository to your local machine.

    Open a terminal or command prompt and navigate to the project directory.

    Build the project by running the following command:

```sh
cargo build --release
```

This will compile the Rust code and generate a production build with optimizations.

After a successful build, you can find the executable file in the target/release directory.

Run the tool by executing the following command:

arduino

./target/release/string_shuffler <input_string>

Replace <input_string> with the string you want to shuffle. The tool will shuffle the characters of the input string and display the shuffled string as output.

For example:

```sh
./target/release/string_shuffler "Hello, World!"
```
Output:
```
    Shuffled string: dHleW,rlo o!
```
License

This project is licensed under the MIT License.

Feel free to use, modify, and distribute this code according to the terms of the license.
Contributing

If you encounter any issues with the tool or have suggestions for improvement, please open an issue on the GitHub repository or submit a pull request. Contributions are welcome!
Acknowledgements

This tool was developed using the Rust programming language and the following libraries:

    rand - for generating random shuffles.
    env - for retrieving command-line arguments.

Special thanks to the Rust community for their invaluable resources and support.
