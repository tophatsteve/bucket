use super::bucket;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

pub trait FileSystem {
    fn get_blob_name(&self, p: &PathBuf) -> String;
    fn get_file_contents(&self, p: &PathBuf) -> Vec<u8>;
    fn encode_file_name(&self, f: &str) -> String;
}

pub struct LocalFileSystem {
    root_folder: String,
}

impl FileSystem for LocalFileSystem {
    fn get_blob_name(&self, p: &PathBuf) -> String {
        let root = Path::new(&self.root_folder);
        let stripped = p.strip_prefix(root).unwrap();
        self.encode_file_name(stripped.to_str().unwrap())
    }

    fn encode_file_name(&self, f: &str) -> String {
        // convert Windows paths to standard format
        let normalized = f.replace("\\", "/");
        utf8_percent_encode(&normalized, DEFAULT_ENCODE_SET).collect()
    }

    fn get_file_contents(&self, p: &PathBuf) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut file = File::open(p).unwrap();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    }
}

impl LocalFileSystem {
    pub fn new(config: &bucket::Config) -> LocalFileSystem {
        LocalFileSystem {
            root_folder: config.root_folder.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[cfg(target_os = "windows")]
    #[test]
    fn test_path_conversion_windows_format() {
        let config = bucket::Config {
            root_folder: String::from("C:/bucket"),
            storage_account: String::from(""),
            account_key: String::from(""),
            root_container_name: String::from(""),
        };

        let fs = LocalFileSystem::new(&config);
        let path = PathBuf::from("C:/bucket\\folder1\\folder2\\file.txt");
        let blob_name = fs.get_blob_name(&path);
        assert_eq!("folder1/folder2/file.txt", blob_name);
    }

    #[test]
    fn test_path_conversion_unix_format() {
        let config = bucket::Config {
            root_folder: String::from("/bucket"),
            storage_account: String::from(""),
            account_key: String::from(""),
            root_container_name: String::from(""),
        };

        let fs = LocalFileSystem::new(&config);
        let path = PathBuf::from("/bucket/folder1/folder2/file.txt");
        let blob_name = fs.get_blob_name(&path);
        assert_eq!("folder1/folder2/file.txt", blob_name);
    }
}
