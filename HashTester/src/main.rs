use sha3::{Digest, Sha3_256, Sha3_384, Sha3_512};
use std::{env, thread, time::Instant};

macro_rules! print_details {
    ($val:expr, $args:expr) => {
        if $args {
            for v in $val {
                println!("{v}");
            }
        }
    };
}

const ITERATIONS: u64 = 1000000;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    let batches = {
        match args.len() {
            1.. => args[0].parse::<u8>().unwrap_or(5),
            _ => 5,
        }
    };

    let total_elapsed = Instant::now();

    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        println!(
            "HashTester
    created to determine how fast different versions of sha3 is
    optional arguments:
        -h | --help       <- shows this menu
        -d | --details    <- shows the speed of each run"
        );
        std::process::exit(0);
    }

    let details = args.contains(&String::from("--details")) || args.contains(&String::from("-d"));

    for i in 1..=batches {
        let job512 = thread::spawn(move || {
            let start = Instant::now();
            let mut local_times = Vec::with_capacity(ITERATIONS as usize + 1);
            local_times.push(String::from("512 times (ns):"));
            for i in 0..ITERATIONS {
                let now = Instant::now();
                let mut hasher = Sha3_512::new();
                hasher.update(format!("{i}").as_bytes());
                local_times.push(format!(
                    "\t{:#?} - {:#?}",
                    now.elapsed().as_nanos(),
                    hasher.finalize()
                ));
            }
            println!("512 finished...");
            print_details!(local_times, details);
            println!(
                "Finished generating {ITERATIONS} sha3 512 hashes in {:#?} ns [{:#?} s]",
                start.elapsed().as_nanos(),
                start.elapsed().as_secs()
            );
        });

        let job384 = thread::spawn(move || {
            let start = Instant::now();
            let mut local_times = Vec::with_capacity(ITERATIONS as usize + 1);
            local_times.push(String::from("384 times (ns):"));
            for i in 0..ITERATIONS {
                let now = Instant::now();
                let mut hasher = Sha3_384::new();
                hasher.update(format!("{i}").as_bytes());
                local_times.push(format!(
                    "\t{:#?} - {:#?}",
                    now.elapsed().as_nanos(),
                    hasher.finalize()
                ));
            }
            println!("384 finished...");
            print_details!(local_times, details);
            println!(
                "Finished generating {ITERATIONS} sha3 384 hashes in {:#?} ns [{:#?} s]",
                start.elapsed().as_nanos(),
                start.elapsed().as_secs()
            );
        });

        let job256 = thread::spawn(move || {
            let start = Instant::now();
            let mut local_times = Vec::with_capacity(ITERATIONS as usize + 1);
            local_times.push(String::from("256 times (ns):"));
            for i in 0..ITERATIONS {
                let now = Instant::now();
                let mut hasher = Sha3_256::new();
                hasher.update(format!("{i}").as_bytes());
                local_times.push(format!(
                    "\t{:#?} - {:#?}",
                    now.elapsed().as_nanos(),
                    hasher.finalize()
                ));
            }
            println!("256 finished...");
            print_details!(local_times, details);
            println!(
                "Finished generating {ITERATIONS} sha3 256 hashes in {:#?} ns [{:#?} s]",
                start.elapsed().as_nanos(),
                start.elapsed().as_secs()
            );
        });

        let _ = job512.join();
        let _ = job384.join();
        let _ = job256.join();
        println!(
            "batch [{i}/{batches}] complete, done in {:#?} ms",
            total_elapsed.elapsed().as_millis()
        );
    }
}
