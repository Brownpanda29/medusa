# Medusa

Medusa is a simple encryptor tool written in Rust that uses AES-256 encryption to encrypt files in specified folders on your system. It can be considered as a tool similar to ransomware for educational purposes.

## Features

- Encrypts files in specified folders using AES-256 encryption.
- Generates an encryption key based on the user's system name and an additional password.
- Skips encryption for specific files (e.g., itself) to avoid encrypting essential system files.
- Saves the encryption key to the user's Desktop after completing encryption.

## Usage

1. Clone or download the Medusa repository to your local machine.

2. Build the project using Cargo:

3. Or  use the Already build binery file in release folder. :)
