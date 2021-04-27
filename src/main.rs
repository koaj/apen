use std::error::Error;

use std::env::args;
use tokio::fs::OpenOptions;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer).await?;
    let mut buffer_vectored: Vec<String> = Vec::new();

    // Push stdin unique inputs to a new vector.
    buffer.lines().for_each(|f| {
        if !buffer_vectored.contains(&f.to_string()) {
            buffer_vectored.push(f.to_owned())
        }
    });

    // Set first argument as file location.
    let args: Vec<String> = args().collect();
    // Read a file to vector
    //
    let mut file_content = String::new();
    let mut appended_content: Vec<String> = Vec::new();
    let mut file_content_unique: Vec<String> = Vec::new();

    if args.len() == 2 {
        // Append to a new file
        let mut new_file = OpenOptions::new()
            .append(true)
            .create(true)
            .read(true)
            .open(&args[1])
            .await?;

        new_file.read_to_string(&mut file_content).await?;

        file_content
            .lines()
            .for_each(|f| file_content_unique.push(String::from(f)));
        buffer_vectored.iter().for_each(|f| {
            if !file_content_unique.contains(f) {
                appended_content.push(f.to_owned())
            }
        });

        for line in appended_content.iter() {
            new_file.write_all(line.as_bytes()).await?;
            new_file.write("\n".as_bytes()).await?;
          
        }
    } else {
        for i in buffer_vectored {
            println!("{}", i)
        }
    }

    // Display results.
    for i in appended_content {
        println!("{}", i)
    }
    Ok(())
}
