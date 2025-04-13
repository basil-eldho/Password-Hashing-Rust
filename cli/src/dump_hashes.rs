use hashassin_core::args::Algorithm;
use scrypt::password_hash::{Encoding, PasswordHash};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::string::String;
use tracing::{error, info};

pub fn dump_hashes(pwd: PathBuf) {
    info!("Dumping hashes: {:?}", pwd);

    let mut file = match File::open(pwd) {
        Ok(f) => f,
        Err(e) => {
            error!("failed to open the file {}", e);
            return;
        }
    };

    let mut buf = Vec::new();

    if file.read_to_end(&mut buf).is_err() {
        error!("Error: Unable to read input file");
        return;
    }

    // Parse the VERSION field
    if buf.is_empty() {
        error!("Error: Input file is too short to contain a valid version.");
        return;
    }

    let version = buf[0];

    // Parse the ALGORITHM LENGTH field
    if buf.len() < 2 {
        error!("Error: Input file is too short to contain a valid algorithm length.");
        return;
    }
    let algorithm_length = buf[1] as usize;

    // Parse the ALGORITHM field
    if buf.len() < 2 + algorithm_length {
        error!("Error: Input file is too short to contain a valid password length.");
        return;
    }

    let algorithm = match String::from_utf8(buf[2..2 + algorithm_length].to_vec()) {
        Ok(a) => Algorithm::from_str(&a),
        Err(_) => {
            error!("Error: Algorithm field contains invalid UTF-8 data.");
            return;
        }
    }
    .unwrap();

    // Parse the PASSWORD LENGTH field
    if buf.len() < 2 + algorithm_length + 1 {
        error!("Error: Input file is too short to contain a valid password length.");
        return;
    }
    let password_length = buf[2 + algorithm_length];

    // Parse the DATA field
    let data_start = 2 + algorithm_length + 1;
    let data = &buf[data_start..];
    info!("Parsed DATA field starting at offset {}", data_start);

    // Output the parsed information
    println!("VERSION: {}", version);
    println!("ALGORITHM: {}", algorithm);
    println!("PASSWORD LENGTH: {}", password_length);

    // Output each hash in the DATA field
    let hash_length = match algorithm {
        Algorithm::Md5 => 16,
        Algorithm::Sha256 => 32,
        Algorithm::Sha512 => 64,
        Algorithm::Scrypt => 88, // scrypt hashes are stored as strings
    };

    let mut offset = 0;
    while offset + hash_length <= data.len() {
        let hash = &data[offset..offset + hash_length];
        match algorithm {
            Algorithm::Md5 | Algorithm::Sha256 | Algorithm::Sha512 => {
                // Print hash in hexadecimal format
                println!("{}", hex::encode(hash));
            }
            Algorithm::Scrypt => match String::from_utf8(hash.to_owned()) {
                Ok(hash) => {
                    let hash_str = PasswordHash::parse(&hash, Encoding::B64)
                        .expect("Unable to parse input file")
                        .to_string();
                    println!("{}", hash_str);
                }
                Err(e) => {
                    error!("Invalid UTF8 with error {}", e);
                    break;
                }
            },
        }
        offset += hash_length;
    }
}
