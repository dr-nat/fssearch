//search from current directory(.)
use std::error::Error;
use std::env;

struct FilePaths {
    file: String,
    file_path: String,
}


fn paths() -> Result<FilePaths, Box<dyn Error>>{
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

fn search(args: &FilePaths) -> Result<(), Box<dyn Error>> {
    let directory = args.file_path.read_dir();

    for files in directory {
        if let Ok(file) = files {
            let file_name = file.file_name();

            if file_name == args.file {
                 file.read_to_string();  
                    file.path()?;
            }
        }
    }
    Ok(())

}



fn main() {
    let result = match paths() {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    
    let file = search(&result);

    println!("searching for {:?}", file);
}
