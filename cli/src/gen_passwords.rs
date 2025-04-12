use rand::distr::Uniform;
use rand::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::SystemTime;
use tracing::info;

pub fn gen_passwords(chars: u8, out_file: Option<PathBuf>, threads: usize, num: usize) {
    let now = SystemTime::now();
    info!(
        "Generating passwords... with parameters: chars: {}, out_file: {:?}, threads: {}, num: {}",
        chars, out_file, threads, num
    );

    let charset = Uniform::new_inclusive(char::from(32), char::from(126)).unwrap();

    let (tx, rx) = mpsc::channel();

    let base_workload = num / threads;
    let extra_workload = num % threads;

    for i in 0..threads {
        let tx = tx.clone();

        let work_load = base_workload + if i < extra_workload { 1 } else { 0 };

        thread::spawn(move || {
            let mut rng = rand::rng();
            for _ in 0..work_load {
                let pwd: String = (0..chars).map(|_| charset.sample(&mut rng)).collect();
                tx.send(pwd).unwrap();
            }
        });
    }

    let mut pwd_list = Vec::new();
    for pwd in rx {
        pwd_list.push(pwd);
    }

    info!("Generated {} passwords.", pwd_list.len());

    println!(
        "Time elapsed till pwd generation: {:?}",
        now.elapsed().unwrap()
    );

    if let Some(file_path) = out_file {
        let file = File::create(file_path).expect("Unable to create file");
        let mut writer = BufWriter::new(file);

        for (i, pwd) in pwd_list.iter().enumerate() {
            if i == pwd_list.len() - 1 {
                write!(writer, "{pwd}").expect("Unable to write to file"); // Avoid last newline
            } else {
                writeln!(writer, "{pwd}").expect("Unable to write to file");
            }
        }
    } else {
        for pwd in pwd_list.iter() {
            println!("{}", pwd);
        }
    }
    info!("Finished generate_passwords function");
    info!("Total time elapsed: {:?}", now.elapsed().unwrap());
}
