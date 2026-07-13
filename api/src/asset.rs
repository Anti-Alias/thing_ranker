use thiserror::Error;

use crate::util::STATIC_FILE_BASE_PATH;

/// Object that allows creating and deleting assets like images.
pub enum AssetStore {
    Local(LocalAssetStore),
    S3(S3AssetStore),
}

impl AssetStore {
    /// Creates an Asset store that uses the local filesystem
    pub fn local() -> Self {
        Self::Local(LocalAssetStore)
    }

    /// Creates an Asset store that uses an s3 bucket
    pub fn s3() -> Self {
        Self::S3(S3AssetStore)
    }

    pub async fn write(&self, path: &str, name: &str, data: &[u8]) -> Result<(), AssetStoreError> {
        match self {
            AssetStore::Local(store) => store.write(path, name, data).await,
            AssetStore::S3(_store) => unimplemented!(),
        }
    }
    pub async fn delete(&self, path: &str) -> Result<(), AssetStoreError> {
        match self {
            AssetStore::Local(store) => store.delete(path).await,
            AssetStore::S3(_store) => unimplemented!(),
        }
    }
}

#[derive(Error, Debug)]
pub enum AssetStoreError {
    #[error("Failed to write asset: {0}")]
    FailedToWriteAsset(Box<dyn std::error::Error>),
    #[error("Failed to delete asset: {0}")]
    FailedToDeleteAsset(Box<dyn std::error::Error>),
}

/// Asset store that uses the local filesystem
pub struct LocalAssetStore;

impl LocalAssetStore {
    pub async fn write(&self, path: &str, name: &str, data: &[u8]) -> Result<(), AssetStoreError> {
        let dir_path = format!("{STATIC_FILE_BASE_PATH}/{path}");
        let full_path = format!("{STATIC_FILE_BASE_PATH}/{path}/{name}");
        tokio::fs::create_dir_all(dir_path)
            .await
            .map_err(|err| AssetStoreError::FailedToWriteAsset(Box::new(err)))?;
        tokio::fs::write(full_path, data)
            .await
            .map_err(|err| AssetStoreError::FailedToWriteAsset(Box::new(err)))
    }
    pub async fn delete(&self, path: &str) -> Result<(), AssetStoreError> {
        let path = format!("{STATIC_FILE_BASE_PATH}/{path}");
        tokio::fs::remove_file(path)
            .await
            .map_err(|err| AssetStoreError::FailedToWriteAsset(Box::new(err)))
    }
}

/// Asset store that uses an S3 bucket.
/// Currently unimplemented.
pub struct S3AssetStore;
