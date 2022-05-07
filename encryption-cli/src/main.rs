use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::{
    env,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};

fn help() {
    println!(
        "encryption-cli
small cli tool to encrypt/decrypt data to/from base64 with a certain key
usage:
    encryption-cli --decrypt key data
    encryption-cli --decrypt key data --file output.txt
    encryption-cli --encrypt key data
    encryption-cli --encrypt key --file input.txt"
    );
    std::process::exit(0);
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO
    // swap to actual arg parser
    let args: Vec<String> = env::args().skip(1).into_iter().collect();
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        help();
    }

    if !(args.len() == 3 || args.len() == 4) {
        help();
    }

    let mut key = String::with_capacity(args[1].len());
    key.clone_from(&args[1]);

    let mc = new_magic_crypt!(&key, 256);

    let file = args.contains(&String::from("--file")) || args.contains(&String::from("-f"));

    if args.contains(&String::from("--encrypt")) || args.contains(&String::from("-e")) {
        let base64 = mc.encrypt_to_base64(&args[2]);
        println!("ecrypted with key: {key}\n{base64}");
        if file {
            let mut output = OpenOptions::new().write(true).create(true).open(key)?;

            writeln!(&mut output, "{:#?}", base64)?;
        }
    } else {
        let data = {
            if file {
                fs::read_to_string(args[2].clone())?
            } else {
                args[2].clone()
            }
        };
        let base64 = mc.decrypt_base64_to_string(data).expect("Invalid Data");
        println!("decrypted with key: {key}\n{base64}");
    }
    Ok(())
}
