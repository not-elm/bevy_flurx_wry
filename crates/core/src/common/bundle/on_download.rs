use std::path::PathBuf;

use bevy::prelude::Component;

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SourceUrl(pub String);


pub(crate) type BoxedDownloadHandler = Box<dyn FnMut(SourceUrl, &mut PathBuf) -> bool + Send + Sync>;


#[derive(Component, Default)]
pub struct OnDownload(Option<BoxedDownloadHandler>);

impl OnDownload {
    pub const NONE: Self = Self(None);

    pub fn new(f: impl FnMut(SourceUrl, &mut PathBuf) -> bool + Send + Sync + 'static) -> Self {
        Self(Some(Box::new(f)))
    }

    #[inline]
    pub(crate) fn take(&mut self) -> Option<BoxedDownloadHandler> {
        self.0.take()
    }
}


