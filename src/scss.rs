use std::{fs, io, path::Path};

/// Compiles every `.scss` file in `src_dir` into a `.css` file with the same
/// name in `out_dir`. Files starting with `_` are treated as partials and skipped.
pub fn compile(src_dir: &str, out_dir: &str) -> io::Result<()> {
    fs::create_dir_all(out_dir)?;

    for entry in fs::read_dir(src_dir)? {
        let path = entry?.path();

        let is_scss = path.extension().is_some_and(|ext| ext == "scss");
        let is_partial = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with('_'));
        if !is_scss || is_partial {
            continue;
        }

        let css = grass::from_path(&path, &grass::Options::default())
            .map_err(|e| io::Error::other(format!("{}: {e}", path.display())))?;

        let out_path = Path::new(out_dir)
            .join(path.file_stem().unwrap())
            .with_extension("css");
        fs::write(&out_path, css)?;
        println!("Compiled {} -> {}", path.display(), out_path.display());
    }

    Ok(())
}
