use std::path::PathBuf;

pub trait Storage {
    fn upload(&self, &PathBuf);
    fn download(&self, &PathBuf);
}

pub struct AzureStorage {}

impl Storage for AzureStorage {
    fn upload(&self, p: &PathBuf) {
        println!("Uploading - {:?}", p);
    }
    fn download(&self, p: &PathBuf) {
        println!("Downloading - {:?}", p);
    }
}
