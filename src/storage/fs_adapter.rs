use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub fn create_folder_fs(folder: &str) -> std::io::Result<()> {
  let project_dirs = ProjectDirs::from("rs", "odam", "odam_data");
  let mut path_buf = PathBuf::from(project_dirs.data_dir());
  path_buf.push(folder);
  fs::create_dir_all(path_buf)?;
  Ok(())
}
