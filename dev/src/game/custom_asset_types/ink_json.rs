use std::str;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub struct InkJson {
    pub string: String,
}

#[derive(Default)]
struct InkJsonAssetLoader;

/// Possible errors that can be produced by [`InkJsonAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
enum InkJsonAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not load file: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

impl AssetLoader for InkJsonAssetLoader {
    type Asset = InkJson;
    type Settings = ();
    type Error = InkJsonAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        info!("Loading InkJson...");
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let string = str::from_utf8(&bytes)?;

        Ok(InkJson {
            string: string.into(),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ink.json"]
    }
}

pub fn plugin(app: &mut App) {
    app.init_asset::<InkJson>();
    app.init_asset_loader::<InkJsonAssetLoader>();
}
