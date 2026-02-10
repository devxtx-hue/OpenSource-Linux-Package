use std::os::unix::fs::PermissionsExt;
use std::fs;
use std::env;
use std::fs::DirEntry;
use std::path::PathBuf;
use colored::*;

/*

fs

created by nowotx

*/

fn main() { // entry point
    output(); // call
}                       


fn read_dir () -> PathBuf { // read dir
    match env::current_dir() {
        Ok(path ) => {
            return path;
        }

        Err(_) => { // if error
            println!("error");
            PathBuf::from(".")
        }
    }
}

fn output() { // output function
    let files = return_files();
    
    for entry in files {
        let filename_os = entry.file_name();
        let filename = filename_os.to_string_lossy();

        if let Ok(metadata) = entry.metadata() {

            let size = metadata.len();

            if metadata.is_dir() {
                println!("directory {}", filename.blue().bold());
            } else {
                let permissions = metadata.permissions();
                let mode = permissions.mode();
                let is_executable = (mode & 0o111) != 0;

                if is_executable {
                    println!("binary {}", filename.green().bold());
                } else {
                    println!("txt {}", filename.white());
                }
            }
        }
    }
}
fn return_files() -> Vec<DirEntry> { // return files
    let dir_name = read_dir();
    let mut files = Vec::new();
    
    match fs::read_dir(&dir_name) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        files.push(entry);
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
    
    files
}