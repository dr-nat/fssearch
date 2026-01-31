//search from current directory(.) 
use fs_search::models::FilePaths;
use fs_search::search::search;
use std::error::Error;
use std::env;

fn entry_paths() -> Result<FilePaths, Box<dyn Error>>{ 
    let args: Vec<String> = env::args().collect(); 
    if args.len() < 2 { 
        return Err(format!("No file paths were provided").into()) 
    } else if args.len()< 3 { 
            return Err(format!("Only one argument was provided").into()) 
        
    } 
    
    let first_arg = args[1].clone(); 

    let second_arg = args[2].clone(); 

    Ok(FilePaths{ file: first_arg, file_path: second_arg }) 
} 


fn main() {
    let result = match entry_paths() {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    
    let file = search(&result); 

    match &file {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
