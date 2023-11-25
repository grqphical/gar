mod archive;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author = "grqphical", version, about, long_about = None)]
struct Cli {
    /// Command to execute
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Creates a new GAR file
    Create {
        files: Vec<String>,

        #[arg(short, long, default_value = "archive.gar")]
        output: String
    },
    /// Lists files in an existing GAR file
    List {
        file: String
    },
    /// Extracts files from a GAR file
    Extract {
        file: String,

        /// Optional directory to output files to
        #[arg(short, long, default_value = "./")]
        output: String
    }
}

fn human_readable_bytes(mut bytes: u32) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut i = 0;
    while bytes >= 1024 && i < units.len() - 1 {
        bytes /= 1024;
        i += 1;
    }

    format!("{} {}", bytes, units[i])
}

/// Main function that returns a result that can be handled
fn try_main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::List { file } => {
            let files = archive::read_archive(file)?;
            // Print a table with all the files in the given archive
            println!("|{:15}|{:15}|{:15}|", "File Name", "Date Modified", "Size");
            println!("{:-<49}", "");
            for file in files {
                println!("|{:<15}|{:<15}|{:<15}|", file.name, file.modification_timestamp, human_readable_bytes(file.size));
            }
        },
        Commands::Create { files, output } => {
            let archive = archive::write_archive(files)?;

            std::fs::write(output, archive)?;
        },
        Commands::Extract { file, output } => {
            let files = archive::read_archive(file)?;

            for file in files {
                let path = format!("{}/{}", output, file.name);

                std::fs::write(path, file.contents)?;
            }
        }
    }

    return Ok(())
}

fn main() {
    match try_main() {
        Ok(_) => (),
        Err(err) => {
            // Print the error and exit gracefully when an error occurs
            eprintln!("{:#}", err);
            std::process::exit(1);
        },
    }
}