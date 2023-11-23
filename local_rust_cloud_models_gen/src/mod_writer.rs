use std::io::Error;
use std::fs::File;

#[derive(Debug)]
pub struct ModWriter {}

impl ModWriter {
    pub fn new() -> ModWriter {
        ModWriter {}
    }

    pub fn register_module(&self, target_path: impl Into<String>, module_name: &str) -> Result<(), Error> {
        
        return Result::Ok(());
    }
}
