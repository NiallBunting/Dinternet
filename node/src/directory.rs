// Ensure the directory structure is correct
//pub fn check_directory() {
//    println!("Checking directory.");
//}

use std::fs;
use std::path::Path;
use home::home_dir;

// Try to create any missing files from the directory
pub fn check_filesystem_or_create() {
    println!("Checking for local directory.");

    match home_dir() {
        None => println!("NO HOME DIRECTORY FOUND"), // TODO(NiallB): Get the user to choose a home directory?
        Some(mut pathbuf) => {
            pathbuf.push("dinternet");

            println!("Your file directory: {:?}", pathbuf);

            // Checks the top level
            find_directory_or_create(&pathbuf).expect("Can't get directory contents.");

            let mut cache_dir = pathbuf.clone();
            cache_dir.push("cache");
            let _ = find_directory_or_create(&cache_dir).expect("Could not create cache dir.");
            cache_dir.push("local");
            let _ = find_directory_or_create(&cache_dir).expect("Could not create cache(local) dir.");
            cache_dir.pop();
            cache_dir.push("public");
            let _ = find_directory_or_create(&cache_dir).expect("Could not create cache(public) dir.");


            let mut data_dir = pathbuf.clone();
            data_dir.push("data");
            let _ = find_directory_or_create(&data_dir).expect("Could not create data dir.");

            let mut private_dir = pathbuf.clone();
            private_dir.push("private");
            let _ = find_directory_or_create(&private_dir).expect("Could not create private dir.");

            let mut public_dir = pathbuf.clone();
            public_dir.push("public");
            let _ = find_directory_or_create(&public_dir).expect("Could not create public dir.");

        },
    }
}

fn find_directory_or_create(pathbuf: &Path) -> Result<std::fs::ReadDir, std::io::Error> {
    match fs::metadata(&pathbuf) {
        Ok(_) => println!("Found: {:?}", pathbuf),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                println!("Could not find directory {:?}. Creating...", pathbuf);
                match fs::create_dir(&pathbuf) {
                    Err(..) => panic!("Can't create directory: {:?}, err {}", pathbuf, e),
                    Ok(..) => println!("Directory created:  {:?}", pathbuf),
                };
            },
            _ => panic!("Can't read directory: {:?}, err {}", pathbuf, e),
        },
    };

    println!("Reading directory: {:?}", pathbuf);
    std::fs::read_dir(pathbuf)
}
