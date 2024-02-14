# Image Cleaner

Image Cleaner is a command-line tool written in Rust that helps you clean up your image folders by deleting specified files and folders.

## Features

- Delete specified files
- Delete specified folders
- Self delete

## Usage

```bash
image_cleaner [OPTIONS] <path>
```

### Options

- `-f`, `--files <files>`: Specify the files to delete
- `-d`, `--dirs <dirs>`: Specify the directories to delete
- `-s`, `--self-delete`: Delete the image_cleaner executable

### Example

```bash
image_cleaner -f "Thumbs.db" -d "cache" "C:\Users\yourusername\Pictures"
```

This command will delete all `Thumbs.db` files and `cache` folders in the `Pictures` folder.

```bash
image_cleaner -s -f "C:\Users\yourusername\Pictures"
```

This command will delete the `image_cleaner` executable and the `Pictures` folder.


## Installation

## From Source
Ensure you have Rust installed on your machine. If not, you can install it from [here](https://www.rust-lang.org/tools/install).

Clone the repository:

```bash
git clone https://github.com/yourusername/image_cleaner.git
cd image_cleaner

cargo build --release

# To run the executable

./target/release/image_cleaner [OPTIONS] <path>

# To install the executable

cargo install --path .

image_cleaner [OPTIONS] <path>
```

## From Binary

Download the binary from the [releases](https://pkg.github.com/yourusername/image_cleaner) page and run it from the command line.

```bash

image_cleaner [OPTIONS] <path>
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```

### Output[]: # Path: LICENSE
```plaintext

MIT License

Copyright (c) 2024

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
IN THE SOFTWARE.
```


### Output[]: # Path: Cargo.toml
```toml


[package] 
name = "image_cleaner"  
version = "0.1.0"
edition = "2024"
