use reqwest::blocking;
use sha2::{Digest, Sha256};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut hasher = Sha256::new();
    hasher.update(
        blocking::get(format!(
            "https://github.com/{}/{}/archive/{}.tar.gz",
            args.get(1)
                .ok_or("requries first argument: <organization-or-user>")?,
            args.get(2)
                .ok_or("requries second argument: <repository>")?,
            args.get(3).ok_or("requires third argument: <tag>")?
        ))?
        .bytes()?,
    );
    println!("{:x}", hasher.finalize());
    Ok(())
}
