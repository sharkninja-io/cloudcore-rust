#[cfg(feature = "library")]
use bytes::Bytes;
#[cfg(feature = "library")]
use serde::de::DeserializeOwned;
#[cfg(feature = "library")]
use std::error::Error;
#[cfg(feature = "library")]
use std::fs::{File, OpenOptions};
#[cfg(feature = "library")]
use std::io::{Cursor, Read, Write};
#[cfg(feature = "library")]
use std::path::Path;
#[cfg(feature = "library")]
use crate::ErrorUtil;

#[cfg(feature = "library")]
pub async fn download_resource(
    url: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file_path);
    if path.exists() {
        return Ok(())
    }
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        let error_payload = response.text().await?;
        return Err(error_payload.into());
    }
    write_to_disk(path, response.bytes().await?)
}

#[cfg(feature = "library")]
pub fn write_to_disk(path: &Path, bytes: Bytes) -> Result<(), Box<dyn Error>> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let mut file = File::create(path)?;
    let mut content = Cursor::new(bytes);
    std::io::copy(&mut content, &mut file)?;
    file.flush()?;
    Ok(())
}

#[cfg(feature = "library")]
pub fn read_from_disk_to_string(path: &str) -> Result<String, Box<dyn Error>>
{

    match OpenOptions::new()
        .read(true).open(path) {
        Ok(mut it) => {
            let mut input_buffer = String::new();
            match it.read_to_string(&mut input_buffer) {
                Ok(_) => Ok(input_buffer),
                Err(err) => {
                    let message: String = format!("{}",err).into();
                    return Err(Box::new(ErrorUtil::disk_read_error(message)))
                } 
            }
        }
        Err(e) => Err(format!("Error::{:?}", e.kind()).into())
    }
}

#[cfg(feature = "library")]
pub fn read_from_disk_to_deserialized<T>(path: &str) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
{

    match read_from_disk_to_string(path) {
        Ok(string_buffer) => {
            let result: T = serde_json::from_str(&string_buffer).unwrap();
            Ok(result)
        }
        Err(err) => {
            let message: String = format!("{}",err).into();
            Err(Box::new(ErrorUtil::disk_read_error(message)))
        }
    }
}

#[cfg(feature = "library")]
pub fn read_from_disk_to_vec(path: &str) -> Result<Vec<u8>, Box<dyn Error>>
{
    match File::open(path) {
        Ok(mut it) => {
            let mut input_buffer = Vec::new();
            match it.read_to_end(&mut input_buffer) {
                Ok(_) => Ok(input_buffer),
                Err(err) => {
                    let message: String = format!("{}",err).into();
                    Err(Box::new(ErrorUtil::disk_error(message)))
                }
            }
        }
        Err(e) => {
            let message: String = format!("Error::{:?}", e.kind()).into();
            Err(Box::new(ErrorUtil::disk_error(message)))
        }
    }
}
