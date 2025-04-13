use crate::Algorithm;
use md5::Md5;
use scrypt::Scrypt;
use scrypt::password_hash::{PasswordHasher, SaltString};
use sha2::{Digest, Sha256};
use sha3::Sha3_512;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, SystemTime};
use std::{io, thread};
use tracing::{error, info};

pub fn gen_hashes(in_file: PathBuf, out_file: PathBuf, threads: usize, algorithm: Algorithm) {
    info!("Generating hashes for passwords({})", algorithm);

    let now = SystemTime::now();

    let file = match File::open(in_file) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open file,{}", e);
            return;
        }
    };

    let buffer_rdr = BufReader::new(file);

    let lines: Vec<String> = buffer_rdr.lines().map_while(Result::ok).collect();

    if lines.is_empty() {
        error!("Input file specified is empty");
        return;
    }

    let pwd_length = lines[0].len();
    for (line_no, line) in lines.iter().enumerate() {
        if line.len() != pwd_length {
            error!(
                "Error: Password on line {} does not conform to the expected length of {}.",
                line_no + 1,
                pwd_length
            );
            return;
        }
    }
    let total_work = lines.len();
    let base_workload = total_work / threads;
    let extra_workload = total_work % threads;

    let (tx, rx) = mpsc::channel();

    for i in 0..threads {
        let tx = tx.clone();
        let algorithm = algorithm.clone();

        let start = i * base_workload + extra_workload.min(i);
        let end = start + base_workload + if i < extra_workload { 1 } else { 0 };

        let lines = lines[start..end].to_vec();

        thread::spawn(move || {
            for line in lines {
                match algorithm {
                    Algorithm::Md5 => {
                        let hash = Md5::digest(line.as_bytes());
                        tx.send(hash.to_vec()).unwrap();
                    }
                    Algorithm::Sha256 => {
                        let hash = Sha256::digest(line.as_bytes());
                        tx.send(hash.to_vec()).unwrap();
                    }
                    Algorithm::Sha512 => {
                        let hash = Sha3_512::digest(line.as_bytes());
                        tx.send(hash.to_vec()).unwrap();
                    }
                    Algorithm::Scrypt => {
                        let salt = SaltString::from_b64("AAAAAAAAAAAAAAAAAAAAAA").unwrap();
                        match Scrypt.hash_password(line.as_bytes(), &salt) {
                            Ok(hash) => tx.send(hash.to_string().into_bytes()).unwrap(),
                            Err(e) => {
                                error!("Failed to hash password with Scrypt: {}", e);
                                continue;
                            }
                        }
                    }
                };
            }
        });
    }

    drop(tx);

    let out_file = match File::create(out_file) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open file,{}", e);
            return;
        }
    };

    let mut out_file_buf = BufWriter::new(out_file);

    // Write VERSION
    if out_file_buf.write_all(&[1]).is_err() {
        error!("Error: Unable to write version to output file.");
        return;
    }

    info!("Written VERSION to output file");

    // Write ALGORITHM LENGTH
    let algorithm_length = algorithm.len() as u8;
    if out_file_buf.write_all(&[algorithm_length]).is_err() {
        error!("Error: Unable to write algorithm length to output file.");
        return;
    }

    info!("Written ALGORITHM LENGTH to output file");

    // Write ALGORITHM
    if out_file_buf.write_all(algorithm.as_bytes()).is_err() {
        error!("Error: Unable to write algorithm to output file.");
        return;
    }

    info!("Written ALGORITHM to output file");

    // Write PASSWORD LENGTH
    if out_file_buf.write_all(&[pwd_length as u8]).is_err() {
        error!("Error: Unable to write password length to output file.");
        return;
    }

    info!("Written PASSWORD LENGTH to output file");

    let tasks_done = Arc::new(Mutex::new(0));
    let c_tasks_done = Arc::clone(&tasks_done);

    thread::spawn(move || {
        let delay = Duration::from_millis(500);
        let mut dots = 0;

        loop {
            let done = *c_tasks_done.lock().unwrap();
            print!(
                "\r\x1b[32mIn progress: {}/{total_work} [{}{}]\x1b[0m",
                done,
                ".".repeat(dots),
                " ".repeat(3 - dots)
            );
            io::stdout().flush().unwrap();

            if done >= total_work {
                break; // Exit the loop when all tasks are done
            }

            dots = (dots + 1) % 4;

            thread::sleep(delay);
        }
    });

    for hash in rx.iter() {
        // Write the hash to the output file
        if out_file_buf.write_all(&hash).is_err() {
            eprintln!("Error: Unable to write hash to output file.");
            return;
        }

        // Update progress bar
        *tasks_done.lock().unwrap() += 1;
    }
    println!(
        "\r\x1b[32mCompleted: {}/{} [Done]\x1b[0m",
        tasks_done.lock().unwrap(),
        total_work
    );
    info!(
        "Finished generate_hashes function: {}s",
        now.elapsed().unwrap().as_secs()
    );
}
