use std::{
    fs::{self, create_dir_all, remove_dir_all},
    path::PathBuf,
};

#[derive(Debug, Clone, Default)]
pub struct ProjectDirs {
    pub(crate) source_dir: PathBuf,
    pub(crate) build_dir: PathBuf,
    pub(crate) dist_dir: PathBuf,
}

impl ProjectDirs {
    pub(crate) fn new_and_create_dir(
        source_dir: PathBuf,
        build_dir: PathBuf,
        dist_dir: PathBuf,
    ) -> Self {
        assert!(matches!(is_dir_empty(&source_dir), Ok(false)));

        create_dir_all(build_dir.clone()).unwrap();
        create_dir_all(dist_dir.clone()).unwrap();
        Self {
            source_dir,
            build_dir,
            dist_dir,
        }
    }

    pub(crate) fn get_source_dir(&self) -> PathBuf {
        self.source_dir.clone()
    }

    pub(crate) fn get_manifest_dir(&self) -> PathBuf {
        self.get_source_dir().join("Cargo.toml")
    }

    pub(crate) fn get_build_dir(&self) -> PathBuf {
        self.build_dir.clone()
    }

    pub(crate) fn clean_all(&self) -> std::io::Result<()> {
        remove_dir_all(self.get_build_dir().clone())?;
        remove_dir_all(self.dist_dir.clone())
    }
}

fn is_dir_empty(dir: &PathBuf) -> Result<bool, std::io::Error> {
    let mut entries = fs::read_dir(dir)?;
    Ok(entries.next().is_none())
}
