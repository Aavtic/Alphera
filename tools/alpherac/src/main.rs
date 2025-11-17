use clap::{Parser, Subcommand};
use std::path::PathBuf;

use lexer::lexer;
use utils::read_from_file;


/// Alphera Compiler
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    //#[clap(short = 'p', long, env)]
    //garden_path: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// compile the source program 
    ///
    /// This command will compile the provided source file.
    Build {
        /// The source file to compile 
        #[clap(short, long)]
        source_file: PathBuf,
    },
}


fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Build{source_file} => {
            let source = handle_reading_file(source_file);
            handle_lexer(source);
        }
    }
}

fn handle_reading_file(source: PathBuf) -> String {
    match read_from_file(source) {
        Ok(contents) => return contents,
        Err(e) => {
            panic!("ERROR: Could not open file due to: {}", e);
        }
    }
}

fn handle_lexer(source: String) {
    let tokens = lexer::lexer::lex(source.as_str());
    for token in tokens {
        println!("{}: {:?}", token.lexeme, token.token_type);
    }
}
