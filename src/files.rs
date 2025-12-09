pub fn create_cmake_file(base_path: &std::path::Path, name: &String, std: &String) {
    let content = format!(
        r#"cmake_minimum_required(VERSION 3.20)
project({} "CXX")

set(CMAKE_CXX_STANDARD {})
set(CMAKE_CXX_STANDARD_REQUIRED true)
set(CMAKE_EXPORT_COMPILE_COMMANDS true)

include_directories(${{CMAKE_SOURCE_DIR}}/include)
file(GLOB_RECURSE src_files CONFIGURE_DEPENDS ${{CMAKE_SOURCE_DIR}}/src/*.cpp)

add_executable(${{PROJECT_NAME}} ${{src_files}})

target_precompile_headers(${{PROJECT_NAME}} PRIVATE ${{CMAKE_SOURCE_DIR}}/include/pch.h)
"#,
        name, std
    );

    let _ = std::fs::write(base_path.join("CMakeLists.txt"), content);
}

pub fn create_main_file(base_path: &std::path::Path) {
    let content = r#"#include "pch.h"

int main()
{
    std::cout << "Hello, world!" << std::endl;
}
"#;

    let _ = std::fs::write(base_path.join("src").join("main.cpp"), content);
}

pub fn create_pch_file(base_path: &std::path::Path) {
    let content = r#"#pragma once

// Include standard headers here to improve build performance.

#include <iostream>
"#;

    let _ = std::fs::write(base_path.join("include").join("pch.h"), content);
}

pub fn create_gitignore_file(base_path: &std::path::Path) {
    let content = r#"build/
vcpkg_installed/"#;

    let _ = std::fs::write(base_path.join(".gitignore"), content);
}
