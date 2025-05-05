use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::thread;
use std::time::Instant;

fn main() -> io::Result<()> {

    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    let key = &args[2];

    let threadn: usize = args[3].parse().unwrap_or_else(|_| {
        eprintln!("Invalid # of threads.");
        4
    });
    
    let file = File::open(args[1].clone())?;

    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let chunk_s = (lines.len() + threadn - 1) /threadn;

    let mut handles = Vec::new();

    for chunk in lines.chunks(chunk_s)
    {
        let chunk = chunk.to_vec();
        let key = key.clone();

        let handle = thread::spawn(move || {
            let mut printing = false;

            for line in chunk
            {
                if line.contains(&key)
                {
                    printing = true;
                    println!("{}", line);
                }

                else if printing && line.starts_with(char::is_whitespace)
                {
                    println!("{}", line);
                }

                else{
                    printing = false;
                }
            }
        });

        handles.push(handle);
    }

    /*for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", line);
    }*/
    for handle in handles{
        handle.join().unwrap();
    }
    //println!("{}", args[1]);

    let duration = start.elapsed();

    println!("{} processed in {} nanoseconds", args[1].clone(), duration.as_nanos());

    Ok(())
}
