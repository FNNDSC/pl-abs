use std::io;
use std::path::{Path, PathBuf};

/// Similar to [chris_plugin.PathMapper.file_mapper](https://fnndsc.github.io/chris_plugin/v0.2.0a1/chris_plugin.html#PathMapper.file_mapper)
pub fn file_mapper<'a, S: AsRef<str>>(
    input_dir: &'a Path,
    output_dir: &'a Path,
    suffixes: &'a [S],
) -> io::Result<impl Iterator<Item = Result<(PathBuf, PathBuf), io::Error>> + 'a> {
    if !input_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("not a directory: {:?}", &input_dir),
        ));
    }
    if !output_dir.is_dir() {
        std::fs::create_dir(&output_dir)?;
    }

    let walk = walkdir::WalkDir::new(&input_dir)
        .into_iter()
        .map(|r| r.map(|entry| entry.into_path()))
        .map(|r| r.map_err(io::Error::from))
        .filter(|r| r.as_ref().map(|p| p.is_file()).unwrap_or(true))
        .filter(move |r| {
            r.as_ref()
                .map(|p| p.file_name().unwrap().to_string_lossy())
                .map(|p| suffixes.iter().any(|s| p.ends_with(s.as_ref())))
                .unwrap_or(true)
        })
        .map(move |r| {
            r.and_then(|input_file| create_out_path_pair(&input_dir, &output_dir, input_file))
        });
    Ok(walk)
}

fn create_out_path_pair(
    input_dir: &Path,
    output_dir: &Path,
    input_file: PathBuf,
) -> Result<(PathBuf, PathBuf), io::Error> {
    let rel = pathdiff::diff_paths(&input_file, input_dir).unwrap();
    let output_file = output_dir.join(rel);
    std::fs::create_dir_all(&output_file.parent().unwrap()).map(|()| (input_file, output_file))
}
