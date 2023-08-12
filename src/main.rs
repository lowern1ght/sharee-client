use std::path::Path;
use clap::{Parser, ValueEnum};
use crossterm::style::Stylize;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TypeSharing {
    Upload,
    Download
}

#[derive(Parser)]
#[command(name = "sharing-client")]
#[command(author = "Al. Sergeev <lowern1ght@yahoo.com>")]
#[command(version = "1.2a")]
#[command(about = "sharing client to send file or download file from sharing server", long_about = None)]
struct Args {
    #[arg(long)]
    path: String,

    #[arg(value_enum)]
    type_sharing: TypeSharing
}

fn upload_file(path: &Path) {
    println!("{}", path.to_str().unwrap())
}

fn download_file(path: &Path) {
    println!("{}", path.to_str().unwrap())
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);
    
    if !path.is_file() {
        println!("path {} exists", Stylize::red("not"));
        return;
    }
    
    match args.type_sharing {
        TypeSharing::Upload => {
            upload_file(path)
        }
        
        TypeSharing::Download => {
            download_file(path)
        }
    }
}
