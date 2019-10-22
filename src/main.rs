use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digest_file() {
        let test_filename: String = String::from("test-file.md");
        let test_file: Vec<u8> = open_file(&test_filename);
        let test_digest: String = digest_file("sha256", test_file);
        let actual_digest: String =
            String::from("E2318D75A3D991E61F47CBD420368816AA25D0BB186785F522F4DD828F6ACA4A");
        assert_eq!(test_digest.to_uppercase(), actual_digest);
    }
}

#[derive(StructOpt)]
struct Cli {
    checksum: Option<String>,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let now: Instant = Instant::now();
    let args = Cli::from_args();
    let path = args.path.into_os_string().into_string().unwrap();
    let checksum: &str = &args.checksum.unwrap_or("sha256".to_string());

    let file_bytes = open_file(&path);
    let file_hash = digest_file(checksum, file_bytes);
    let new_now: Instant = Instant::now();
    println!("{}", &path);
    println!("{}: {}", checksum, file_hash);
    println!(
        "time elapsed: {}ms",
        new_now.duration_since(now).as_millis()
    );
}

fn open_file(path: &String) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    let file = File::open(path).expect("unable to open file");
    let mut buffer = BufReader::new(file);
    buffer.read_to_end(&mut data).expect("unable to read file");
    return data;
}

fn digest_file(digest: &str, bytes: Vec<u8>) -> String {
    match digest {
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.input(bytes);
            let result = hasher.result();
            let result_string = format!("{:x}", result);
            return result_string;
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.input(bytes);
            let result = hasher.result();
            let result_string = format!("{:x}", result);
            return result_string;
        }
        _ => panic!("no algorithm supplied, aborting"),
    }
}
