use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;

/// BufReader for reading file with more descriptive message on error.
pub fn open(file: &Path) -> Result<BufReader<File>> {
	log::info!("loading {}", file.to_string_lossy());
	Ok(BufReader::new(File::open(file).map_err(|err| anyhow!("open {:?}: {}", file, err))?))
}

// Read a file entirely and return the contents.
//fn read_file(file: &Path) -> Result<Vec<u8>> {
//	let mut buf = Vec::new();
//	open(file)?.read_to_end(&mut buf)?;
//	Ok(buf)
//}

/// BufWriter for writing file with more descriptive message on error.
/// Create parent directory if needed.
pub fn create(file: impl AsRef<Path>) -> Result<BufWriter<File>> {
	let file = file.as_ref();
	log::info!("writing {}", file.to_string_lossy());
	if let Some(parent) = file.parent() {
		let _ = mkdir(parent);
	}
	Ok(BufWriter::new(File::create(file).map_err(|err| anyhow!("create {:?}: {}", file, err))?))
}

/// Read file names (no full path) in a directory.
pub fn read_dir_names(path: &Path) -> Result<impl Iterator<Item = PathBuf>> {
	Ok(fs::read_dir(path)
		.map_err(|e| anyhow!("read '{path:?}': {e}"))? //
		.filter_map(|entry| entry.ok())
		.map(|entry| PathBuf::from(entry.file_name())))
}

pub fn mkdir(path: impl AsRef<Path>) -> Result<()> {
	let path = path.as_ref();
	fs::create_dir(path).map_err(|e| anyhow!("create directory '{path:?}': {e}"))
}

/// Equivalent of "rm -rf": remove file/directory, succeed if it did not exist in the first place.
pub fn force_remove(path: impl AsRef<Path>) -> Result<()> {
	let path = path.as_ref();

	if !path.exists() {
		return Ok(());
	}

	if path.is_dir() {
		std::fs::remove_dir_all(path)?
	} else {
		std::fs::remove_file(path)?
	}

	match path.exists() {
		false => Ok(()),
		true => Err(anyhow!("failed to delete {path:?}")),
	}
}
