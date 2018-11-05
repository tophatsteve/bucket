# bucket
A Dropbox style service written in Rust and backed by Azure blob storage.

[![Build Status](https://travis-ci.com/tophatsteve/bucket.svg?branch=master)](https://travis-ci.com/tophatsteve/bucket)
[![Coverage Status](https://coveralls.io/repos/github/tophatsteve/bucket/badge.svg?branch=master)](https://coveralls.io/github/tophatsteve/bucket?branch=master)

## Setup

bucket needs the following environment variables setting in order to work:

- SENTRY_DSN - bucket uses Sentry to log errors, so it needs a Sentry DSN to connect with.
- STORAGE_ACCOUNT - The name of the Azure Storage Account to use.
- STORAGE_MASTER_KEY - The key used to connect to the Azure Storage Account.
- STORAGE_CONTAINER - The name of the container in the Azure Storage Account where files will be stored.
- ROOT_FOLDER - The folder on the local machine where files will be stored. Anything put in here will be uploaded to the Azure Storage Account.


## Features

- [x] Upload individual files to blob storage
- [x] Upload folders to blob storage
- [x] Delete individual files from blob storage
- [x] Delete folders from blob storage
- [ ] Monitor blob storage account for changes
- [ ] Download new files from blob storage
- [ ] Download new folders from blob storage
- [ ] Remove local files that have been removed from blob storage
- [ ] Remove local folders that have been removed from blob storage