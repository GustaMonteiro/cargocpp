pub fn new(name: &String, std: &String) {
    println!("Creating {name} project in C++ {std}");
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
