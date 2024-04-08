# Medusa

![Medusa Logo](logo.png)

Medusa is a simple encryptor tool written in Rust that uses AES-256 encryption to encrypt files in specified folders on your system. It can be considered as a tool similar to ransomware for educational purposes.

## Features

- Encrypts files in specified folders using AES-256 encryption.
- Generates an encryption key based on the user's system name and an additional password.
- Skips encryption for specific files (e.g., itself) to avoid encrypting essential system files.
- Saves the encryption key to the user's Desktop after completing encryption.

## Usage

1. Clone or download the Medusa repository to your local machine.

2. Build the project using Cargo:

3. Run the Medusa executable:

4. Medusa will encrypt files in specified folders on your system. You can customize the list of folders and other settings in the source code (`main.rs`) as needed.

## Disclaimer

Medusa is a tool created for educational purposes only. It is not intended for malicious use. The authors do not condone or support any illegal or unethical activities.

## License

This project is licensed under the [MIT License](LICENSE).

## Credits

Medusa was created by [Ethan Prime] and is maintained by Me .
