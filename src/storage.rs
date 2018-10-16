use std::path::PathBuf;

pub trait Storage {
    fn upload(&self, &PathBuf);
}

pub struct AzureStorage {}

impl Storage for AzureStorage {
    fn upload(&self, p: &PathBuf) {
        println!("Uploading - {:?}", p);
    }
}
