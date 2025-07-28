use std::{
    fs::{File, remove_file},
    io::Write,
    path::PathBuf,
};

use reqwest::Response;

use crate::indexing::cache::init_cache;
use color_eyre::Result;

pub async fn cache_remote_objects_inv(
    url: &str,
    project_name: String,
    maybe_cache_path: Option<PathBuf>,
    force: bool,
) -> Result<()> {
    let cache_path = init_cache(maybe_cache_path)?
        .join("sphinx")
        .join(PathBuf::from(project_name.to_lowercase()))
        .with_extension("inv");

    if cache_path.exists() {
        if force {
            remove_file(&cache_path)?;
        } else {
            // object is already cached so we don't have to do anything else
            return Ok(());
        }
    }

    let response = fetch_objects_inv_blocking(url).await?;
    let data = response.bytes().await?;

    let mut file = File::create(cache_path)?;

    file.write_all(&data)?;
    Ok(())
}

pub async fn fetch_objects_inv_blocking(url: &str) -> Result<Response> {
    Ok(reqwest::get(url).await?)
}

#[cfg(test)]
mod test {

    use std::fs::{File, create_dir_all, exists};

    use assert_fs::TempDir;
    use color_eyre::Result;
    use walkdir::WalkDir;

    use crate::indexing::fetch::cache_remote_objects_inv;

    #[tokio::test]
    async fn cache_clean_numpy_obj_inv() -> Result<()> {
        let url = "https://numpy.org/doc/stable/objects.inv";

        let tmp_dir = TempDir::new()?;
        cache_remote_objects_inv(
            url,
            "numpy".to_string(),
            Some(tmp_dir.path().to_path_buf()),
            false,
        )
        .await?;

        assert!(exists(
            tmp_dir
                .path()
                .join("sphinx")
                .join("numpy")
                .with_extension("inv")
        )?);

        Ok(())
    }

    #[tokio::test]
    async fn test_cache_force() -> Result<()> {
        let url = "https://numpy.org/doc/stable/objects.inv";

        let tmp_dir = TempDir::new()?;
        let cache_path = tmp_dir.path().join("sphinx");
        let obj_path = cache_path.join("numpy.inv");
        create_dir_all(&cache_path)?;

        let _ = File::create(&obj_path)?;

        cache_remote_objects_inv(
            url,
            "numpy".to_string(),
            Some(tmp_dir.path().to_path_buf()),
            false,
        )
        .await?;

        {
            let file_size = std::fs::metadata(&obj_path)?.len();
            assert!(file_size == 0);
        }

        cache_remote_objects_inv(
            url,
            "numpy".to_string(),
            Some(tmp_dir.path().to_path_buf()),
            true,
        )
        .await?;

        for entry in WalkDir::new(tmp_dir.path()).follow_links(true) {
            let entry = entry?;
            println!("{}", entry.path().display());
        }
        {
            let file_size = std::fs::metadata(&obj_path)?.len();
            assert!(file_size > 0);
        }

        Ok(())
    }
}
