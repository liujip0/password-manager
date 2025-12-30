# liujip0-password-manager

VERY INSECURE simple password manager CLI application written in Rust. This was written as a hobby project, and please do not use it to store actual passwords!

All passwords are stored in a `.toml` file in the home directory. The passwords are XOR'd with the user's master password before storing. This is not a secure way to store passwords as any single leaked password can easily be used to determine the master key.

Note: The so-called "master password" CAN be different for each password, although this may cause problems for exporting and importing.
