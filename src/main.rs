use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use clap::{Arg, Command};

// Read file function
fn read_file(filename: &str) -> io::Result<Vec<u8>> {
    // Open the file in read-only mode (ignoring errors)
    let file = File::open(filename)?;

    // Create a buffered reader to read the file efficiently
    let mut buf_reader = BufReader::new(file);

    // Create a vector to hold the file's contents
    let mut buffer = Vec::new();

    // Read the entire file into the buffer
    buf_reader.read_to_end(&mut buffer)?;

    // Return the buffer
    Ok(buffer)
}

// Write file function
fn write_file(filename: &str, data: &[u8]) -> io::Result<()> {
    // Open the file in write-only mode, create the file if it doesn't exist
    let file = File::create(filename)?;

    // Create a buffered writer to write the file efficiently
    let mut buf_writer = BufWriter::new(file);

    // Write the data to the file
    buf_writer.write_all(data)?;

    // Ensure all data is written to the disk
    buf_writer.flush()?;

    Ok(())
}

// Compress data function
fn compress_data(data: &[u8]) -> io::Result<Vec<u8>> {
    // Create a buffer to hold the compressed data
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    // Write the data to the encoder
    encoder.write_all(data)?;

    // Finish the compression process and retrieve the compressed data
    let compressed_data = encoder.finish()?;

    Ok(compressed_data)
}

// Decompress data function
fn decompress_data(data: &[u8]) -> io::Result<Vec<u8>> {
    // Create a buffer to hold the decompressed data
    let mut decoder = GzDecoder::new(data);

    // Create a vector to hold the decompressed data
    let mut decompressed_data = Vec::new();

    // Read the decompressed data into the vector
    decoder.read_to_end(&mut decompressed_data)?;

    Ok(decompressed_data)
}

// Build the CLI
fn build_cli() -> Command {
    Command::new("file_comp_decomp") // Creates a new CLI command called file_comp_decomp
        .version("1.0") // Sets the version of the CLI app
        .about("Compresses and decompresses files") // Sets about description for the CLI app
        .arg(
            Arg::new("compress") // Defines new arg for compressing
                .short('c')
                .long("compress")
                .action(clap::ArgAction::SetTrue)
                .help("Compress the input file"),
        )
        .arg(
            Arg::new("decompress") // Defines new arg for decompressing
                .short('d')
                .long("decompress")
                .action(clap::ArgAction::SetTrue)
                .help("Decompress the input file"),
        )
        .arg(
            Arg::new("input") // Defines new arg for input file
                .short('i')
                .long("input")
                .required(true)
                .help("Input file"),
        )
        .arg(
            Arg::new("output") // Defines new arg for output file
                .short('o')
                .long("output")
                .required(true)
                .help("Output file"),
        )
}

fn main() {
    // Parse the command-line arguments
    let matches = build_cli().get_matches();

    // Get the input and output file names
    let input_file = matches.get_one::<String>("input").expect("Invalid arg missing");
    let output_file = matches.get_one::<String>("output").expect("Invalid arg missing");

    // Debug prints to verify arguments
    println!("Input file: {}", input_file);
    println!("Output file: {}", output_file);

    // Get the flag values
    let compress = matches.get_flag("compress");
    let decompress = matches.get_flag("decompress");

    // Ensure only one of compress or decompress is set
    if compress && decompress {
        eprintln!("Please specify either --compress or --decompress, not both.");
        std::process::exit(1);
    }

    // Check if the compress flag is present
    if compress {
        println!("Compress flag is set");
        // File compression
        match read_file(input_file) {
            Ok(data) => {
                // Compress the data
                match compress_data(&data) {
                    Ok(compressed_data) => {
                        // Write the compressed data to an output file
                        match write_file(output_file, &compressed_data) {
                            Ok(()) => println!("File compressed and written successfully!"),
                            Err(e) => eprintln!("Failed to write compressed file: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Failed to compress data: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to read file: {}", e),
        }
    } else if decompress {
        println!("Decompress flag is set");
        // File decompression
        match read_file(input_file) {
            Ok(compressed_data) => {
                // Decompress the data
                match decompress_data(&compressed_data) {
                    Ok(decompressed_data) => {
                        // Write the decompressed data to an output file
                        match write_file(output_file, &decompressed_data) {
                            Ok(()) => println!("File decompressed and written successfully!"),
                            Err(e) => eprintln!("Failed to write decompressed data: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Failed to decompress data: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to read compressed data: {}", e),
        }
    } else {
        // Print an error message if neither compress nor decompress flag is present
        eprintln!("Please specify either --compress or --decompress");
    }
}
