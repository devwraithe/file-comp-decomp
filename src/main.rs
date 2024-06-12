use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

// Read file function
fn read_file(filename: &str) -> io::Result<Vec<u8>> {
    // Open the file in read-only mode (ignoring errors)
    let file = File::open(filename)?;

    // Create a buffered reader to read the file effeciently
    let mut buf_reader = BufReader::new(file);

    // Create a vector to hold the file's contents
    let mut buffer = Vec::new();

    // Read the entire file into the buffer
    buf_reader.read_to_end(&mut buffer)?;

    // Print the contents of the file as a string
    println!("{}", String::from_utf8_lossy(&buffer));

    // Return the buffer
    Ok(buffer)
}

// Write file function
fn write_file(filename: &str, data: &[u8]) -> io::Result<()> {
    // Open the in write-only mode, create the file if it doesn't exist
    let file = File::create(filename)?;

    // Create a buffered write to write to the file efficiently
    let mut buf_writer = BufWriter::new(file);

    // Write the data to the file
    buf_writer.write_all(data)?;

    // Ensure all data is written to the disk
    buf_writer.flush()?;
    
    // Return the buffer
    Ok(())
}

// Compress data function
fn compress_data(data: &[u8]) -> io::Result<Vec<u8>>{
    // Create a buffer to hold the compressed data
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    // Write the data to the encoder
    encoder.write_all(data)?;

    // Finish the compression process and retrieve the compressed data
    let compressed_data = encoder.finish()?;

    Ok(compressed_data)
}

fn main() {
    // Read the input file
    match read_file("example.txt") {
        Ok(data) => {
            // Compress the data
            match compress_data(&data) {
                Ok(compressed_data) => {
                    // Write the compressed data to an output file
                    match write_file("compressed.gz", &compressed_data) {
                        Ok(()) => println!("File compressed and written successfully!"),
                        Err(e) => eprintln!("Failed to write compressed file: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to compress data: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}
