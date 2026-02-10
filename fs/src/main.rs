use std::fs;
use std::env;
use std::os::unix::fs::PermissionsExt;
use colored::*;

/*

fs

created by nowotx

*/

fn main() {
    let path = env::current_dir().unwrap();

    for i in fs::read_dir(&path).unwrap() {
        let element = i.unwrap();
        let file_name_os = element.file_name();
        let file_name = file_name_os.to_string_lossy();

        if let Ok(metadata) = element.metadata(){
            if metadata.is_dir(){
                println!("{}", file_name.white().bold());
        } else {
                let permissions = metadata.permissions();
                let mode = permissions.mode();
                let is_executable = (mode & 0o111) != 0;  // 0o111 = --x--x--x

                if is_executable {
                    println!("{}", file_name.green().bold());
                }
        }
    }
}
}                       