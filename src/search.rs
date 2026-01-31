use bytesize::ByteSize;
use chrono::prelude::*;
use std::fs; 
use std::error::Error; 
use crate::models::FilePaths;

pub fn search(args: &FilePaths) -> Result<(), Box<dyn Error>> { 
    let directory = fs::read_dir(&args.file_path)?; 

    let mut search_tracker = false; 

    for files in directory { 

        if let Ok(entry) = files { 
            let file_name = &entry. file_name(); 

            //Implementing the search functionality(The Search engine). 

            if file_name.to_string_lossy().into_owned() == args.file { 

                search_tracker = true; 

                println!("\nFile_name: {:?}", file_name); 

                let file_metadata = &entry.metadata()?; 

                if file_metadata.is_file() { 

                    println!("\n{:?}", &file_metadata.file_type()); 
                } 

                if file_metadata.is_file() { 
                    let file_size = &file_metadata.len();
                    println!("\nFile_size: {}", ByteSize(*file_size)); 
                } 

                // use of the chrono time crate to convert sytem time to human readable date time.
                if file_metadata.is_file() {
                     let modified_time = &file_metadata.modified()?;

                     let modified: DateTime<Local> = (*modified_time).into();
                     println!("\nModified_time: {}", modified.format("%Y-%m-%d %H:%M:%S")); 

                } 

               if file_metadata.is_file() { 
                   println!("\n{:?}", &file_metadata.permissions()); 

               } 

               if file_metadata.is_file() { 

                   let accessed_time = &file_metadata.accessed()?;

                   let access: DateTime<Local> = (*accessed_time).into(); 
                   println!("\nAccessed_time: {}", access.format("%Y-%m-%d %H:%M:%S")); 

               }
               if file_metadata.is_file() {
                    let creation_time = &file_metadata.created()?;

                    let created_time: DateTime<Local> = (*creation_time).into();
                    println!("\n Created on: {}", created_time.format("%Y-%m-%d %H:%M:%S"));

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
            println!("File not found.");
       }


        Ok(())
}

