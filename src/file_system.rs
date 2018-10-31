use super::bucket;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

pub trait FileSystem {
    fn get_blob_name(&self, p: &PathBuf) -> String;
    fn get_file_contents(&self, p: &PathBuf) -> Vec<u8>;
}

pub struct LocalFileSystem {
    root_folder: String,
}

impl FileSystem for LocalFileSystem {
    fn get_blob_name(&self, p: &PathBuf) -> String {
        let root = Path::new(&self.root_folder);
        let stripped = p.strip_prefix(root).unwrap();
        // convert Windows paths to standard format
        let normalized = stripped.to_str().unwrap().replace("\\", "/");
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
