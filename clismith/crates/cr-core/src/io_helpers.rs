use noyalib::compat::serde_yaml;
use serde::Serialize;
use std::{fs::File, io::BufWriter, path::Path};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreIoError {
    #[error("File reading error occured: {0}")]
    IoReadError(std::io::Error),
    #[error("File writing error occured: {0}")]
    IoWriteError(std::io::Error),
    #[error("Json serialization error occured: {0}")]
    JsonError(serde_json::Error),
    #[error("Yaml serialization error occured: {0}")]
    YamlError(noyalib::compat::serde_yaml::Error),
    #[error("Unknown extension for deserialization: {0}")]
    UnknownExt(String),
}

pub fn deserialize_from_file<T: for<'de> serde_core::de::Deserialize<'de> + 'static>(
    file: &Path,
) -> std::result::Result<T, CoreIoError> {
    //println!("{}", file.to_string_lossy());
    let ext = file.extension().unwrap_or_default().to_string_lossy();
    if ext == "json" {
        return deserialize_json_from_file(file);
    } else if ext == "yml" || ext == "yaml" {
        return deserialize_yaml_from_file(file);
    }

    Err(CoreIoError::UnknownExt(ext.to_string()))
}

pub fn serialize_to_file<T>(obj: &T, file: &Path) -> std::result::Result<(), CoreIoError>
where
    T: Serialize,
{
    //println!("{}", file.to_string_lossy());
    let ext = file.extension().unwrap_or_default().to_string_lossy();
    if ext == "json" {
        return serialize_json_to_file(obj, file);
    } else if ext == "yml" || ext == "yaml" {
        return serialize_yaml_to_file(obj, file);
    }

    Err(CoreIoError::UnknownExt(ext.to_string()))
}

fn deserialize_json_from_file<T: for<'de> serde_core::de::Deserialize<'de> + 'static>(
    file: &Path,
) -> std::result::Result<T, CoreIoError> {
    let src = std::fs::read_to_string(file).map_err(CoreIoError::IoReadError)?;
    let result = serde_json::from_str(&src).map_err(CoreIoError::JsonError)?;
    Ok(result)
}

fn deserialize_yaml_from_file<T: for<'de> serde_core::de::Deserialize<'de> + 'static>(
    file: &Path,
) -> std::result::Result<T, CoreIoError> {
    let src = std::fs::read_to_string(file).map_err(CoreIoError::IoReadError)?;
    let result = serde_yaml::from_str(&src).map_err(CoreIoError::YamlError)?;
    Ok(result)
}

fn serialize_json_to_file<T>(obj: &T, file: &Path) -> std::result::Result<(), CoreIoError>
where
    T: Serialize,
{
    let file = File::create(file).map_err(CoreIoError::IoWriteError)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &obj).map_err(CoreIoError::JsonError)?;
    Ok(())
}

fn serialize_yaml_to_file<T>(obj: &T, file: &Path) -> std::result::Result<(), CoreIoError>
where
    T: Serialize,
{
    let file = File::create(file).map_err(CoreIoError::IoWriteError)?;
    let writer = BufWriter::new(file);
    serde_yaml::to_writer(writer, &obj).map_err(CoreIoError::YamlError)?;
    Ok(())
}
