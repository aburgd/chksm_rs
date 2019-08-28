use structopt::StructOpt;
use std::path::PathBuf;
use std::io::{Read, BufReader};
use std::fs::File;
use sha2::{Sha256, Sha512, Digest};

#[derive(StructOpt)]
struct Cli {
    checksum: String,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let path = args.path
        .into_os_string()
        .into_string()
        .unwrap();

    let file_bytes = open_file(path);
    let file_hash = digest_file(args.checksum, file_bytes);
    println!("{}", file_hash);
}

fn open_file(path: String) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    let file = File::open(path).expect("unable to open file");
    let mut buffer = BufReader::new(file);
    buffer.read_to_end(&mut data).expect("unable to read file");
    return data;
}

fn digest_file(digest: String, bytes: Vec<u8>) -> String {
    match digest.as_ref() {
       "sha256" => {
            let mut hasher = Sha256::new();
            hasher.input(bytes);
            let result = hasher.result();
            let result_string = format!("{:x}", result);
            return result_string;
        },
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.input(bytes);
            let result = hasher.result();
            let result_string = format!("{:x}", result);
            return result_string;
        },
        _ => panic!("no algorithm supplied, aborting"),
    }
}