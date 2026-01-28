//search from current directory(.) 
use std::fs; 
use std::error::Error; 
use std::env; 
struct FilePaths { 
    file: String, 
    file_path: String, 
} 

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


fn search(args: &FilePaths) -> Result<(), Box<dyn Error>> { 
    let directory = fs::read_dir(&args.file_path)?; 

    let mut search_tracker = false; 

    for files in directory { 

        if let Ok(entry) = files { 
            let file_name = &entry. file_name(); 

            //Implementing the search functionality(The Search engine). 

            if file_name.to_string_lossy().into_owned() == args.file { 

                search_tracker = true; 

                println!("\n{:?}", file_name); 

                let file_metadata = &entry.metadata()?; 

                if file_metadata.is_file() { 

                    println!("\n{:?}", &file_metadata.file_type()); 
                } 

                if file_metadata.is_file() { 
                    println!("\n{:?}", &file_metadata.len()); 

                } 

                if file_metadata.is_file() {
                     println!("\n{:?}", &file_metadata.modified()?); 

               } 

               if file_metadata.is_file() { 
                   println!("\n{:?}", &file_metadata.permissions()); 

               } 

               if file_metadata.is_file() { 

                   println!("\n{:?}", &file_metadata.accessed()?); 

              } 

              let path_buf = &entry.path(); 

              let file_contents = fs::read_to_string(path_buf)?; 

              println!("\n{:?}", file_contents);

            } else if entry.metadata()?.is_dir() {
                let sub_dir = entry.path();
                    
                let sub_args = FilePaths {
                    file: sub_dir.to_string_lossy().into_owned(),
                    file_path: args.file_path.clone(),
                };

                search(&sub_args)?;
            } 
        }
    }     

    if search_tracker == false {
        println!("File now found.");
    }
            
    Ok(())
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
