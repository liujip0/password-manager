# liujip0-password-manager

VERY INSECURE simple password manager CLI application written in Rust. This was written as a hobby project, and please do not use it to store actual passwords!

All passwords are stored in a `.toml` file in the home directory. The passwords are XOR'd with the user's master password before storing. This is not a secure way to store passwords as any single leaked password can easily be used to determine the master key.

Note: The so-called "master password" CAN be different for each password, although this may cause problems for exporting and importing.

## Local testing

You can follow these instructions, or you can download the [./liujip0-password-manager.exe](./liujip0-password-manager.exe) file.

### 1. Clone this repository

```zsh
gh repo clone liujip0/password-manager
```

```zsh
git clone https://github.com/liujip0/password-manager.git
```

### 2. Install Rust

Follow these instructions: <https://doc.rust-lang.org/book/ch01-01-installation.html>

### 3. Build the application

```zsh
cargo build --release
```

### 4. Run the application

```zsh
./target/release/liujip0-password-manager.exe
```
