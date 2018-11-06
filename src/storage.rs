use super::bucket;
use azure_sdk_for_rust::core::errors::AzureError;
use azure_sdk_for_rust::core::DeleteSnapshotsMethod;
use azure_sdk_for_rust::prelude::*;
use futures::future::*;
use hyper::StatusCode;
use std::path::PathBuf;
use tokio_core::reactor::Core;

#[derive(Debug, Fail)]
pub enum StorageError {
    #[fail(display = "The specified path was not found")]
    PathNotFound,
    #[fail(display = "Could not connect to the server - {:?}", error)]
    ConnectionError { error: AzureError },
    #[fail(display = "An unknown error has occurred - {:?}", error)]
    UnknownError { error: AzureError },
}

pub trait Storage {
    fn upload(&self, &str, Vec<u8>);
    fn download(&self, &PathBuf);
    fn delete(&self, &str) -> Result<(), StorageError>;
    fn list_folder_blobs(&self, &str) -> Vec<String>;
}

pub struct AzureStorage {
    pub storage_account: String,
    pub account_key: String,
    pub root_container_name: String,
}

impl Storage for AzureStorage {
    fn upload(&self, blob_name: &str, data: Vec<u8>) {
        trace!("Uploading - {:?}", blob_name);

        let mut core = Core::new().unwrap();
        let client = Client::new(&self.storage_account, &self.account_key).unwrap();

        let digest = md5::compute(&data[..]);

        let future = client
            .put_block_blob()
            .with_container_name(&self.root_container_name)
            .with_blob_name(blob_name)
            .with_body(&data[..])
            .with_content_md5(&digest[..])
            .finalize();

        core.run(future).unwrap();
    }

    fn download(&self, p: &PathBuf) {
        trace!("Downloading - {:?}", p);
    }

    fn delete(&self, blob_name: &str) -> Result<(), StorageError> {
        trace!("Deleting - {:?}", blob_name);

        let mut core = Core::new().unwrap();
        let client = Client::new(&self.storage_account, &self.account_key).unwrap();

        let future = client
            .delete_blob()
            .with_container_name(&self.root_container_name)
            .with_blob_name(blob_name)
            .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
            .finalize();

        let result = core.run(future);

        match result {
            Err(AzureError::UnexpectedHTTPResult(ref h))
                if h.status_code() == StatusCode::NOT_FOUND =>
            {
                Err(StorageError::PathNotFound)
            }
            Err(e) => {
                trace!("Error deleting {} - {:?}", blob_name, e);
                Err(StorageError::UnknownError { error: e })
            }
            Ok(_) => Ok(()),
        }
    }

    fn list_folder_blobs(&self, blob_name: &str) -> Vec<String> {
        let mut blobs = Vec::<String>::new();
        let folder_name = format!("{}/", blob_name);
        let mut core = Core::new().unwrap();
        let client = Client::new(&self.storage_account, &self.account_key).unwrap();
        let future = client
            .list_blobs()
            .with_container_name(&self.root_container_name)
            .with_prefix(&folder_name)
            .finalize()
            .map(|iv| {
                for blob in iv.incomplete_vector.iter() {
                    blobs.push(blob.name.clone());
                }
                blobs
            });

        let result = core.run(future);

        match result {
            Err(_) => Vec::<String>::new(),
            Ok(o) => o,
        }
    }
}

impl AzureStorage {
    pub fn new(config: &bucket::Config) -> AzureStorage {
        AzureStorage {
            storage_account: config.storage_account.clone(),
            account_key: config.account_key.clone(),
            root_container_name: config.root_container_name.clone(),
        }
    }

    fn list_container_contents(&self) {
        let mut core = Core::new().unwrap();
        let client = Client::new(&self.storage_account, &self.account_key).unwrap();
        let future = client
            .list_blobs()
            .with_container_name(&self.root_container_name)
            .finalize()
            .map(|iv| {
                println!("List blob returned {} blobs.", iv.incomplete_vector.len());
                for cont in iv.incomplete_vector.iter() {
                    println!(
                        "\t{}\t{} B\t{:?}\t{:?}",
                        cont.name, cont.content_length, cont.last_modified, cont.content_type
                    );
                }
            });

        core.run(future).unwrap();
    }
}
