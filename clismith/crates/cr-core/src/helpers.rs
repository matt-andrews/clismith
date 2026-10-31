use std::{fs::File, io::BufWriter, path::Path};

use noyalib::compat::serde_yaml;
use serde::Serialize;

pub fn deserialize_yaml_from_path<T: for<'de> serde_core::de::Deserialize<'de> + 'static>(
    dir: &Path,
    file: &str,
) -> std::result::Result<T, noyalib::compat::serde_yaml::Error> {
    let src = std::fs::read_to_string(format!("{}/{}", dir.display(), file));
    if let Err(e) = src {
        eprintln!(
            "{}",
            format_args!(
                "error: can't read {}/{}: {}",
                dir.display(),
                file,
                e.to_string()
            )
        );
        return Err(noyalib::compat::serde_yaml::Error::from(
            noyalib::Error::Deserialize("failed to deserialize".to_string()),
        ));
    }

    serde_yaml::from_str(&src.unwrap())
}

pub fn serialize_to_json<T>(obj: &T, output: &Path) -> anyhow::Result<()>
where
    T: Serialize,
{
    let file = File::create(output)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &obj)?;
    Ok(())
}
