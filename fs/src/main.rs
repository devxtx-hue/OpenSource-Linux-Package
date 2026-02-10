
use std::fs::metadata;
use std::os::unix::fs::PermissionsExt;
use std::fs;
use std::env;
use std::fs::DirEntry;
use std::path::PathBuf;
use colored::*;


const EXECUTABLE_MASK: u32 = 0o111;


/*

fs

created by nowotx

*/


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let target_path = if args.len() > 1 {
        Some(args[1].as_str())
    } else {
        None
    };
    
    output(target_path);
}                  

fn format_size(bytes: u64) -> String { // get size
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn get_current_dir(target_path: Option<&str>) -> PathBuf { // dir
    match target_path {
        Some(path_str) => {
            let path = PathBuf::from(path_str);
            if path.is_dir() {
                path
            } else {
                eprintln!("Error: '{}' is not a directory, using current dir", path_str);
                env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
            }
        }
        None => {
            env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        }
    }
}

fn output(target_path: Option<&str>) {
    let files = return_files(target_path);
    let dir_name = get_current_dir(target_path);
    
    let mut entries: Vec<(String, bool, bool, u64)> = Vec::new(); 
    
    for entry in &files {
        let filename = entry.file_name().to_string_lossy().to_string();
        
        if let Ok(metadata) = entry.metadata() {
            let is_dir = metadata.is_dir();
            let is_exec = (metadata.permissions().mode() & EXECUTABLE_MASK) != 0;
            let size = metadata.len();

            entries.push((filename, is_dir, is_exec, size));
        }
    }
    
    entries.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    

    let chunk_size = (entries.len() + 2) / 3; 
    
    for i in 0..chunk_size {
        for col in 0..3 {
            let idx = i + col * chunk_size;
            if idx < entries.len() {
                let (name, is_dir, is_exec, size) = &entries[idx];
                
                let display_name = if *is_dir {
                    format!("{}/", name).blue().bold()
                } else if *is_exec {
                    name.green().bold()
                } else {
                    name.white()
                };

                print!("{:<25}", display_name) //, format_size(*size));
            }
        }
        println!();
    }
}

fn return_files(target_path: Option<&str>) -> Vec<DirEntry> {
    let dir_name = get_current_dir(target_path);
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