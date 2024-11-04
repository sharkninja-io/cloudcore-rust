#[cfg(feature = "library")]
use std::collections::HashMap;
#[cfg(feature = "library")]
use std::fs;
#[cfg(feature = "library")]
use std::fs::{create_dir_all, read_dir};
use std::path::{Path, PathBuf};
#[cfg(feature = "library")]
use bytes::Bytes;
#[cfg(feature = "library")]
use serde::Deserialize;
#[cfg(feature = "library")]
use log::{debug, error};
#[cfg(feature = "library")]
use crate::cloudcore::CACHE_APP_DIR;
#[cfg(feature = "library")]
use crate::io::{read_from_disk_to_string, write_to_disk};
#[cfg(feature = "library")]
use crate::urls::CRATE_WORKSPACE;
#[cfg(feature = "library")]
use crate::ErrorUtil;
#[cfg(feature = "library")]
use mantle_utilities::{ErrorType, MantleError};

use std::error::Error;
use std::sync::Mutex;
use serde::Serialize;
use serde_json::{Value};

#[cfg(feature = "library")]
static CACHE_HIDDEN_FILE_NAME: &str = "/.store";

#[derive(Serialize, Debug)]
pub enum CacheDataValue {
    StringValue(String),
    IntegerValue(i32),
    DoubleValue(f64),
    BooleanValue(bool),
    ObjectValue(Value),
    NullValue
}

pub trait CacheInteract {
    /// Retrieve data contents and serde_json::Value which returns
    /// a object casted as a Value which can be casted to its necessary type.
    /// ```
    /// let value = cache.get_value("path", "key")?;
    /// println!("{:?}", value.as_str());
    /// ```
    /// It is completely safe to unwrap since it can return None which avoids the lib to panic.
    fn get_value(&self, path: String, key: String) -> Result<CacheDataValue, Box<dyn Error>>;

    /// Set data to the mutable content. You can pass in any type which can be
    /// serde::Serialize since it needs to be converted at some point to
    /// serde_json::Value to be stored.
    fn set_value<T>(&mut self, path: String, key: String, value: T) -> Result<(), Box<dyn Error>> where T: Serialize;

    /// Remove the value for the key at the specified path. If the path does not exist
    /// an error is returned. If the key does not exist it is a NoOp
    fn remove_value(&mut self, path: String, key: String) -> Result<(), Box<dyn Error>>;
}

pub trait CacheDir {
    /// If the parent_dir exist and its been given
    /// by the consuming OS we can then make the
    /// directory with the given path. This only has
    /// to run once, however it calls recursively the create_dir_all.
    /// Once the child path has been set it gets inserted
    /// into the map for Cache for constant time lookup and easy storage.
    fn make_dir_for_child<'a>(&mut self, path: &'a str) -> Result<PathBuf, Box<dyn Error>>;

    /// If the parent_dir exist and its been given
    /// by the consuming OS we can then delete the
    /// directory with the given path. This only has
    /// to run once, however it calls recursively the remove_dir_all.
    /// Once the child path has been deleted it gets removed
    /// from the map.
    fn remove_dir_for_child<'a>(&mut self, path: &'a str) -> Result<(), Box<dyn Error>>;

    /// Remove key from child_paths
    fn remove_child_path(&mut self, key: String);

    /// This will generate a default file but will be reused with replacing data.
    /// In the module we can read and mutate content but then the
    /// file needs to be recreated. This does not hurt performance
    /// or cause any poor behavior. All file is always up to date and no file or data is lingering.
    fn touch_file_for_child(&self, child_dir: &Path, bytes: Option<String>) -> Result<(), Box<dyn Error>>;

    /// Read the contents of the files stored in the cache directory by a
    /// given path. Retrieve as a io buffer stream and return a
    /// string construct from it for easy parsing of data to structs or T types.
    fn stream_buffer_from_child<'a>(&self, path: &'a str) -> Result<String, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct Cache {
    /// Contains the path constructed with
    /// the OS File directory path with our module path.
    /// Returns a reference of the Path -> Slice [u8]
    #[cfg(feature = "library")]
    parent_path: PathBuf,

    /// Child paths will hold all child dirs which
    /// uses the parent as the entire dir.
    /// ```../parent_path/child_paths...```
    #[cfg(feature = "library")]
    child_paths: HashMap<String, PathBuf>,
    #[allow(dead_code)]
    lock: Mutex<usize>,
}

#[cfg(feature = "library")]
#[derive(Debug, Serialize, Deserialize)]
struct CacheData {
    /// Data will hold all the persisted objects in storage. Its the entry point to our cache.
    /// ```
    /// {
    ///   "data"; {
    ///     // ... cache.
    ///   }
    /// }
    /// ```
    data: HashMap<String, Value>,
}

#[cfg(feature = "library")]
// Constructor
impl Cache {
    /// In order to construct this object we need to pass in the
    /// OS file directory where we can have access to
    /// read and write to persist our data. We pass in the OS file dir
    /// as a String and then it gets formatted into a path to be consumed by the lib
    pub fn new(os_dir: String) -> Result<Self, Box<dyn Error>> {
        if os_dir.is_empty() {
            return Err(Box::new(ErrorUtil::path_empty_error()));
        }

        let uri = format!("{}/{}", &os_dir, CRATE_WORKSPACE);
        create_dir_all(&uri)?;

        let file_dir = Path::new(&uri).to_path_buf();
        if !file_dir.exists() {
            let error = format!("Malformed path or incorrect path was given -> {:?}", &file_dir);
            return Err(Box::new(ErrorUtil::malformed_or_incorrect_path(error)));
        }

        let mut instance = Self { parent_path: file_dir, child_paths: HashMap::new(), lock: Mutex::new(0) };
        debug!("Parent Path: {:?}", &instance.parent_path.as_path());

        let mut children = HashMap::new();
        match read_dir(&uri) {
            Ok(child_paths) => {
                for path in child_paths {
                    match path {
                        Ok(entry) => {
                            match entry.file_name().into_string() {
                                Ok(file_name) => {
                                    children.insert(file_name, entry.path());
                                },
                                Err(err) => error!("Error getting file_name from {:?} : {:?}", entry.file_name(), err)
                            }
                        }
                        Err(err) => error!("Error reading directory Entry: {}", err.to_string())
                    }
                }
            },
            Err(err) => error!("Error reading directory {}: {}", &uri, err.to_string())
        }
        instance.child_paths = children;

        Ok(instance)
    }
    pub fn delete(&mut self) {
        for (child, path) in &self.child_paths {
            if child != CACHE_APP_DIR {
                if path.is_dir() {
                    if let Some(err) = fs::remove_dir_all(path).err() {
                        error!("Error deleting child path {}: {}", child, err.to_string());
                    }
                }
            }
        }
        let app_child = self.child_paths.remove(CACHE_APP_DIR);
        self.child_paths = HashMap::new();
        if let Some(ac) = app_child {
            self.child_paths.insert(CACHE_APP_DIR.to_string(), ac);
        }
    }
}

#[cfg(feature = "library")]
// Borrowed Getters
impl Cache {
    /// Get a reference to the parent path which is the OS file dir.
    pub fn parent_path(&self) -> &Path {
        return self.parent_path.as_path();
    }

    /// Get a reference to the child map which is a construct
    /// of dirs with the parent as the point of entry.
    pub fn child_paths(&mut self) -> &HashMap<String, PathBuf> {
        return &self.child_paths;
    }
}

#[cfg(feature = "library")]
// OS and File Dir Handlers
impl CacheDir for Cache {
    fn make_dir_for_child<'a>(&mut self, path: &'a str) -> Result<PathBuf, Box<dyn Error>> {
        if path.is_empty() {
            return Err(Box::new(ErrorUtil::path_empty_error()));
        }

        let parent_dir = self.parent_path();
        if !parent_dir.exists() {
            return Err(Box::new(ErrorUtil::parent_directory_missing()));
        }
        let parent_dir_str = parent_dir.to_str();
        if parent_dir_str.is_none() {
            return Err("Could not get parent directory as a string".into());
        }
        let mut uri = parent_dir_str.unwrap().to_string();
        uri.push_str("/");
        uri.push_str(path);
        create_dir_all(&uri)?;

        let child_dir = Path::new(&uri);
        if !child_dir.exists() {
            let error = format!("{:?}", &child_dir);
            return Err(Box::new(ErrorUtil::malformed_or_incorrect_path(error)));
        }
        debug!("Child Path: {:?}", &child_dir);

        let path_buf = &child_dir.to_path_buf();

        self.touch_file_for_child(child_dir, None)?;

        self.child_paths.insert(path.to_string(), path_buf.to_owned());

        Ok(path_buf.to_owned())
    }

    fn remove_dir_for_child<'a>(&mut self, path: &'a str) -> Result<(), Box<dyn Error>> {
        if path.is_empty() {
            return Err(Box::new(ErrorUtil::path_empty_error()));
        }

        let parent_dir = self.parent_path();
        if !parent_dir.exists() {
            return Err(Box::new(ErrorUtil::parent_directory_missing()));
        }

        match fs::remove_dir_all(path) {
            Ok(_) => {
                self.child_paths.remove(path);
                Ok(())
            }
            Err(err) => Err(Box::new(MantleError {
                error_type: ErrorType::DiskError,
                description: err.to_string()
            }))
        }
    }

    fn remove_child_path(&mut self, key: String) {
        self.child_paths.remove(&key);
    }

    fn touch_file_for_child(&self, child_dir: &Path, bytes: Option<String>) -> Result<(), Box<dyn Error>> {
        if !child_dir.exists() {
            return Err(Box::new(ErrorUtil::path_empty_error()));
        }
        let child_dir_str = child_dir.to_str();
        if child_dir_str.is_none() {
            return Err("Could not get child directory as a string".into());
        }
        let mut uri = child_dir_str.unwrap().to_string();
        uri.push_str(CACHE_HIDDEN_FILE_NAME);

        let mut object = String::from(r#"{"data":{}}"#);
        match bytes {
            Some(b) => {  object = b; }
            None => {}
        }
        write_to_disk(Path::new(&uri), Bytes::from(object.into_bytes()))
    }

    fn stream_buffer_from_child<'a>(&self, path: &'a str) -> Result<String, Box<dyn Error>> {
        if let Some(hash_path) = self.child_paths.get(path) {
            if let Some(hash_path_str) = hash_path.to_str() {
                let mut child = hash_path_str.to_string();
                child.push_str(CACHE_HIDDEN_FILE_NAME);
                read_from_disk_to_string(&child)
            } else {
                Err("Could not get hash path as a string to stream buffer from child".into())
            }
        } else {
            Err("Could not get hash path to stream buffer from child".into())
        }
    }
}

#[cfg(feature = "library")]
// User Facing Handlers
impl CacheInteract for Cache {
    fn get_value(&self, path: String, key: String) -> Result<CacheDataValue, Box<dyn Error>> {
        match self.lock.lock() {
            Ok(_) => {
                if path.is_empty() {
                    return Err(Box::new(ErrorUtil::path_empty_error()));
                }

                if key.is_empty() {
                    return Err(Box::new(ErrorUtil::cache_key_missing()));
                }

                if self.child_paths.contains_key(&path) {
                    let io_buffer = self.stream_buffer_from_child(&path)?;

                    let cache: CacheData = serde_json::from_str(&io_buffer)?;
                    let value = cache.data.get(&key).unwrap_or(&Value::Null);

                    let result = match value {
                        Value::Null => { CacheDataValue::NullValue }
                        Value::Bool(it) => {
                            CacheDataValue::BooleanValue(*it)
                        }
                        Value::Number(it) => {
                            if it.is_i64() {
                                let int = it.as_i64();
                                if let Some(integer) = int {
                                    let i = integer as i32;
                                    CacheDataValue::IntegerValue(i)
                                } else {
                                    return Err(Box::new(ErrorUtil::int_parse_error()));
                                }
                            } else if it.is_f64() {
                                let dbl = it.as_f64();
                                if let Some(dub) = dbl {
                                    CacheDataValue::DoubleValue(dub)
                                } else {
                                    return Err(Box::new(ErrorUtil::double_parse_error()));
                                }
                            } else {
                                CacheDataValue::NullValue
                            }
                        }
                        Value::String(it) => {
                            CacheDataValue::StringValue(it.to_owned())
                        }
                        Value::Array(it) => {
                            let val = serde_json::to_value(it)?;
                            CacheDataValue::ObjectValue(val)
                        }
                        Value::Object(it) => {
                            let val = serde_json::to_value(it)?;
                            CacheDataValue::ObjectValue(val)
                        }
                    };

                    Ok(result)
                } else {
                    return Err(Box::new(ErrorUtil::child_directory_missing()));
                }
            }
            Err(err) => {
                let e = err.to_string();
                return Err(e.into())
            }
        }
    }

    fn set_value<T>(&mut self, path: String, key: String, value: T) -> Result<(), Box<dyn Error>> where T: Serialize {
        match self.lock.lock() {
            Ok(_) => {
                if path.is_empty() {
                    return Err(Box::new(ErrorUtil::path_empty_error()));
                }

                if self.child_paths.contains_key(&path) {
                    if let Some(hash_path) = self.child_paths.get(&path) {
                        if let Some(hash_path_str) = hash_path.to_str() {
                            let mut child = hash_path_str.to_string();
                            child.push_str(CACHE_HIDDEN_FILE_NAME);

                            let io_buffer = self.stream_buffer_from_child(&path)?;

                            let mut cache: CacheData = serde_json::from_str(&io_buffer)?;
                            let value = serde_json::to_value(&value)?;
                            cache.data.insert(key.to_string(), value);

                            let out_buffer = serde_json::to_string(&cache)?;

                            self.touch_file_for_child(hash_path, Some(out_buffer))?;

                            Ok(())
                        } else {
                            Err("Could not get hash path as a string to set value".into())
                        }
                    } else {
                        Err("Could not get hash path to set value".into())
                    }
                } else {
                    return Err(Box::new(ErrorUtil::child_directory_missing()));
                }
            }
            Err(err) => {
                let e = err.to_string();
                return Err(e.into())
            }
        }
    }

    fn remove_value(&mut self, path: String, key: String) -> Result<(), Box<dyn Error>> {
        match self.lock.lock() {
            Ok(_) => {
                if path.is_empty() {
                    return Err(Box::new(ErrorUtil::path_empty_error()));
                }

                if self.child_paths.contains_key(&path) {
                    if let Some(hash_path) = self.child_paths.get(&path) {
                        if let Some(hash_path_str) = hash_path.to_str() {
                            let mut child = hash_path_str.to_string();
                            child.push_str(CACHE_HIDDEN_FILE_NAME);

                            let io_buffer = self.stream_buffer_from_child(&path)?;

                            let mut cache: CacheData = serde_json::from_str(&io_buffer)?;
                            if cache.data.remove(&key).is_none() {
                                debug!("No value for '{}' in cache", key);
                            } else {
                                let out_buffer = serde_json::to_string(&cache)?;
                                self.touch_file_for_child(hash_path, Some(out_buffer))?;
                            }
                        } else {
                            return Err("Could not get hash path as a string to remove value".into())
                        }
                    } else {
                        return Err("Could not get hash path to remove value".into())
                    }
                    Ok(())
                } else {
                    return Err(Box::new(ErrorUtil::child_directory_missing()));
                }
            }
            Err(err) => {
                let e = err.to_string();
                return Err(e.into())
            }
        }
    }
}
