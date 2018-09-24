use std::path::PathBuf

pub trait Storage {
    pub fn upload(&self, &PathBuf);
}

pub struct AzureStorage {}

impl Storage for AzureStorage {
    pub fn upload(&self, p: &PathBuf) {
        println!("Uploading - {:?}", p);
    }
}