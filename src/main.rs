//benchmark stuff ig
use std::{
    time::{Instant, Duration},
    fs::File,
    io::Write, 
};

struct Config {
    val: i64,
}

fn main() -> std::io::Result<()> {
    let config = Config { val: 100_000_000_000 };
    let listen = Instant::now();
    let mut sum = 0.0;
    let max_duration = Duration::new(10, 0); 

    for i in 0..config.val {
        if listen.elapsed() >= max_duration {
            println!("Stopping after 10 seconds...");
            break;
        }

        let x = i as f64;
        let y = (x * 1.5).sin();
        let z = (y * 2.0).cos();
        sum += z;
    }

    let duration = listen.elapsed();
    let mut file = File::create("results.txt")?;
    file.write_all(format!("Completed in: {:?}\n", duration).as_bytes())?;
    file.write_all(format!("Final sum: {}\n", sum).as_bytes())?;
    println!("Completed in {:?}", duration);
    println!("Final sum: {}", sum);

    Ok(())
}
