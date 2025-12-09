use std::process::exit;
use std::path::PathBuf;

use crate::files::*;

pub fn new(name: &String, std: &String) {
    let base_path = PathBuf::from(name);

    if std::fs::exists(&base_path).unwrap() {
        println!("There is already a directory called {}", base_path.display());
        exit(-1);
    }

    println!("Creating {} project in C++ {}", base_path.display(), std);

    if let Err(e) = std::fs::create_dir(&base_path) {
        println!("Something went wrong during directory creation: {}", e);
        exit(-1);
    }

    std::fs::create_dir(base_path.join("src")).expect("Error while creating src directory");
    std::fs::create_dir(base_path.join("include")).expect("Error while creating include directory");
    create_main_file(&base_path);
    create_pch_file(&base_path);
    create_cmake_file(&base_path, name, std);
    create_gitignore_file(&base_path);
}

pub fn build(clean: &bool) {
    if *clean {
        crate::routines::clean();
    }

    println!("Building project...");
}

pub fn clean() {
    println!("Cleaning project...");
}

pub fn run(quiet: &bool) {
    println!(
        "Running project{}",
        if *quiet { " in quiet mode" } else { "" }
    )
}

pub fn add(dependencies: &Vec<String>) {
    if dependencies.len() > 0 {
        println!(
            "Adding {} dependencies: {:?}",
            dependencies.len(),
            dependencies
        );
    }
}
