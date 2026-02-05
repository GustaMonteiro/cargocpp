use json::{self, JsonValue};

pub struct CmakeTargetFile {
    content: JsonValue,
}

impl CmakeTargetFile {
    pub fn from(json_str: &str) -> CmakeTargetFile {
        let parsed = json::parse(json_str);

        if parsed.is_err() {
            panic!("Cmake target file parsing error");
        }

        Self {
            content: parsed.unwrap(),
        }
    }

    pub fn has(&self, package_name: &str) -> bool {
        self.content.has_key(package_name)
            && self.content[package_name].has_key("release")
            && self.content[package_name].has_key("debug")
    }

    fn get(&self, package_name: &str, build_mode: &str) -> Option<String> {
        if !self.has(package_name) {
            return None;
        }

        Some(self.content[package_name][build_mode].to_string())
    }

    pub fn get_release_for(&self, package_name: &str) -> Option<String> {
        self.get(package_name, "release")
    }

    pub fn get_debug_for(&self, package_name: &str) -> Option<String> {
        self.get(package_name, "debug")
    }
}
