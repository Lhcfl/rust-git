//! Models of git
//!
//! models represent entities that will eventually be stored in the file system
//! (they are [Store](Store)s) or need to be kept in memory at
//! runtime. You should not put specific interactive functions here except
//! loading and saving from disk.

pub mod blob;
pub mod branch;
pub mod commit;
pub mod head;
pub mod ignores;
pub mod object;
pub mod repo;
pub mod stage;
pub mod tree;

use std::{
    io,
    marker::PhantomData,
    path::{Path, PathBuf},
};

pub trait DirContainer {
    const DIRECTORY: &'static str;

    fn make_dir(root: &Path) -> io::Result<()> {
        let path = root.join(Self::DIRECTORY);
        std::fs::create_dir_all(path)
    }

    fn check_dir_exists(root: &Path) -> bool {
        let path = root.join(Self::DIRECTORY);
        path.exists()
    }
}

pub trait Store
where
    Self: Sized,
{
    fn location(&self) -> PathBuf;
    fn store(&self, root: &Path) -> io::Result<()>;
    fn load(file: &Path) -> io::Result<Self>;
    fn delete(&self, root: &Path) -> io::Result<()> {
        let path = root.join(self.location());
        std::fs::remove_file(path)
    }
}

#[macro_export]
macro_rules! serde_json_store {
    () => {
        fn store(&self, root: &std::path::Path) -> std::io::Result<()> {
            let path = root.join(self.location());
            if let Some(parent) = path.parent() {
                // Safely ignores the error if the directory already exists
                let _ = std::fs::create_dir_all(parent);
            }
            std::fs::write(
                path,
                serde_json::to_string(self)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
            )
        }
        fn load(path: &std::path::Path) -> std::io::Result<Self> {
            let data = std::fs::read(path)?;
            let inner = serde_json::from_slice(&data)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            Ok(inner)
        }
    };
}

/// Wrapped the accessor to the storable object
pub struct Accessor<'a, By, T>
where
    T: Store,
    T: Accessible<By>,
{
    by: &'a By,
    _will_into: PhantomData<T>,
}

/// The trait for the object that can be accessed by the accessor
pub trait Accessible<By>
where
    Self: Store,
{
    /// Get an accessor of the object
    fn accessor<'a>(by: impl Into<&'a By>) -> Accessor<'a, By, Self> {
        Accessor {
            by: by.into(),
            _will_into: PhantomData,
        }
    }

    fn path_of(by: &By) -> PathBuf;
}

impl<By, T> Accessor<'_, By, T>
where
    T: Store,
    T: Accessible<By>,
{
    pub fn path(&self) -> PathBuf {
        T::path_of(self.by)
    }
}
