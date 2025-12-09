use std::path::PathBuf;
use std::process::exit;

use crate::files::*;

fn initialize_git_repo(base_path: &std::path::Path) {
    std::process::Command::new("git")
        .arg("init")
        .current_dir(&base_path)
        .status()
        .expect("Error while creating git repository");

    std::process::Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&base_path)
        .status()
        .expect("Error while changing git branch name");
}

pub fn new(name: &String, std: &String) {
    let base_path = PathBuf::from(name);

    if std::fs::exists(&base_path).unwrap() {
        println!(
            "There is already a directory called {}",
            base_path.display()
        );
        exit(-1);
    }

    if let Err(e) = std::fs::create_dir(&base_path) {
        println!("Something went wrong during directory creation: {}", e);
        exit(-1);
    }

    initialize_git_repo(&base_path);

    std::fs::create_dir(base_path.join("src")).expect("Error while creating src directory");
    std::fs::create_dir(base_path.join("include")).expect("Error while creating include directory");
    create_main_file(&base_path);
    create_pch_file(&base_path);
    create_cmake_file(&base_path, name, std);
    create_gitignore_file(&base_path);
}

pub fn build(clean: &bool) {
    if !std::fs::exists("CMakeLists.txt").unwrap() {
        println!("No CMake file found in the current directory");
        exit(-1);
    }

    if *clean {
        crate::routines::clean();
    }

    if !std::fs::exists("build").unwrap() {
        std::fs::create_dir("build").expect("Error while creating build directory");
        std::process::Command::new("cmake")
            .args(["-B", "build", "-S", "."])
            .status()
            .expect("Something went wrong when configuring cmake project");
    }

    std::process::Command::new("cmake")
        .args(["--build", "build"])
        .status()
        .expect("Something went wrong when build cmake project");
}

pub fn clean() {
    if !std::fs::exists("build").unwrap() || !std::fs::exists("CMakeLists.txt").unwrap() {
        println!("No project structure found to clean");
        exit(-1);
    }

    std::fs::remove_dir_all("build").expect("Something went wrong when deleting build directory");
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
