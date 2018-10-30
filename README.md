# bucket
A Dropbox style service written in Rust and backed by Azure blob storage

## Setup

bucket needs the following environment variables setting in order to work:

- SENTRY_DSN - bucket uses Sentry to log errors, so it needs a Sentry DSN to connect with.
- STORAGE_ACCOUNT - The name of the Azure Storage Account to use.
- STORAGE_MASTER_KEY - The key used to connect to the Azure Storage Account.
- STORAGE_CONTAINER - The name of the container in the Azure Storage Account where files will be stored.
- ROOT_FOLDER - The folder on the local machine where files will be stored. Anything put in here will be uploaded to the Azure Storage Account.