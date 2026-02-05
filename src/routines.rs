use std::path::PathBuf;
use std::process::{Stdio, exit};

use crate::files::*;

fn check_command(command: &str) {
    if let Err(_) = std::process::Command::new(command)
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
    {
        println!("Command '{command}' not found");
        exit(-1);
    }
}

fn initialize_git_repo(base_path: &std::path::Path) {
    check_command("git");

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

pub fn build(clean: &bool, release: &bool) {
    build_internal(clean, &false, release);
}

fn build_internal(clean: &bool, quiet: &bool, release: &bool) {
    if !std::fs::exists("CMakeLists.txt").unwrap() {
        println!("No CMake file found in the current directory");
        exit(-1);
    }

    if *clean {
        crate::routines::clean();
    }

    check_command("cmake");

    let stdout = if *quiet {
        Stdio::null()
    } else {
        Stdio::inherit()
    };
    let stderr = if *quiet {
        Stdio::null()
    } else {
        Stdio::inherit()
    };

    let build_type = if *release { "Release" } else { "Debug" };

    if !std::fs::exists("build").unwrap() {
        std::fs::create_dir("build").expect("Error while creating build directory");
        std::process::Command::new("cmake")
            .args([
                "-B",
                "build",
                "-S",
                ".",
                &format!("-DCMAKE_BUILD_TYPE={}", build_type),
            ])
            .stdout(stdout)
            .stderr(stderr)
            .status()
            .expect("Something went wrong when configuring cmake project");
    }

    std::process::Command::new("cmake")
        .args(["--build", "build", "--config", build_type])
        .stdout(if *quiet {
            Stdio::null()
        } else {
            Stdio::inherit()
        })
        .stderr(if *quiet {
            Stdio::null()
        } else {
            Stdio::inherit()
        })
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

pub fn run(quiet: &bool, release: &bool, args: &Vec<String>) {
    if !std::fs::exists("CMakeLists.txt").unwrap() {
        println!("No CMake file found in the current directory");
        exit(-1);
    }

    build_internal(&false, quiet, release);

    let cmake_content =
        std::fs::read_to_string("CMakeLists.txt").expect("Failed to read CMakeLists.txt");

    let project_name = cmake_content
        .lines()
        .find(|line| line.trim().starts_with("project("))
        .and_then(|line| {
            line.split('(')
                .nth(1)?
                .split_whitespace()
                .next()?
                .split(')')
                .next()
        })
        .expect("Could not find project name in CMakeLists.txt");

    // Construct the path to the executable
    // Try to find the executable in different locations to support multi-configuration generators (like MSVC)
    let build_type = if *release { "Release" } else { "Debug" };
    let possible_paths = vec![
        PathBuf::from("build").join(project_name), // Unix-like (Makefiles)
        PathBuf::from("build").join(build_type).join(project_name), // Multi-config (MSVC, Ninja Multi-Config)
        PathBuf::from("build")
            .join(build_type)
            .join(format!("{}.exe", project_name)), // Windows MSVC
    ];

    let executable_path = possible_paths
        .into_iter()
        .find(|path| std::fs::exists(path).unwrap_or(false))
        .unwrap_or_else(|| {
            println!("Executable not found. Checked paths:");
            println!("  - build/{}", project_name);
            println!("  - build/{}/{}", build_type, project_name);
            exit(-1);
        });

    let status = std::process::Command::new(&executable_path)
        .args(args)
        .status()
        .expect("Failed to execute the binary");

    if !status.success() {
        exit(status.code().unwrap_or(-1));
    }
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
