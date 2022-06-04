# pwgen
A simple password generator written in rust.

## Compilation

### Linux
If on Linux / WSL:
```sh
cargo build --release
```
Cross platform / verbose:
```sh
cargo build --release --target x86_64-unknown-linux-gnu
```

### Windows
If on Windows:
```sh
cargo build --release
```

Cross platform / verbose:
```sh
cargo build --release --target x86_64-pc-windows-gnu
```
If you are on a Linux-based system, make sure that you have run the following:
```sh
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
```
The first tells `cargo` that a new target architecture is availible, and specifies *what* the toolchain is.
The second command actually installs the toolchain components. 

## Usage
When run in a terminal on Linux, or by double-clicking `pwgen.exe` in a Windows file explorer the following prompt will display:
```
Enter service name:
_
```
At this point, enter whatever phrase you wish to associate with the service that the password is for.
The program will then prompt for a length. Enter this and press enter, and the program will print the generated password to the console.
This can then be copied and pasted into whatever application needs the password.

Given the same input, the program will always produce the same password, 
so it can be used to store passwords somewhat securely, 
however the program is not designed for this so the security of the storage is up to you.

## Maintenence
Since the program works and dosen't have issues that I am aware of, this release is more-or-less prepetual. 
If there is some super urgent security concern that I am made aware of, I will fix them and post a new version.
I plan to eventuall make a release for Mac, but don't have any way to test it and don't want to add to the list of current broken software. 
