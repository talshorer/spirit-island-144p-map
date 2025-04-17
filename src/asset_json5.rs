use std::{
    io,
    marker::PhantomData,
    str::{self, Utf8Error},
};

use bevy::{
    asset::{Asset, AssetLoader},
    reflect::TypePath,
};
use serde::Deserialize;
use thiserror::Error;

pub(crate) struct Json5AssetLoader<A>
where
    A: TypePath + Send + Sync,
{
    _phantom: PhantomData<A>,
    extensions: Vec<&'static str>,
}

impl<A> Json5AssetLoader<A>
where
    A: TypePath + Send + Sync,
{
    pub(crate) fn new(extensions: &[&'static str]) -> Self {
        Self {
            _phantom: PhantomData,
            extensions: extensions.to_owned(),
        }
    }
}

#[derive(Error, Debug)]
pub(crate) enum ConfigAssetLoaderError {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("utf8: {0}")]
    Utf8(#[from] Utf8Error),
    #[error("json5: {0}")]
    Json5(#[from] json5::Error),
}

impl<A> AssetLoader for Json5AssetLoader<A>
where
    for<'a> A: TypePath + Send + Sync + Asset + Deserialize<'a>,
{
    type Asset = A;

    type Settings = ();

    type Error = ConfigAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let s = str::from_utf8(&bytes)?;
        Ok(json5::from_str(s)?)
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
