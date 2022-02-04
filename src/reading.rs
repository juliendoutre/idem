use std::error::Error;

#[derive(Debug)]
pub struct FileExtensionError {
    path: String,
}

impl std::fmt::Display for FileExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}'s file extension is not `.id`", self.path)
    }
}

impl Error for FileExtensionError {}

pub fn read(path: &str) -> Result<String, Box<dyn Error>> {
    if path.ends_with(".id") {
        match std::fs::read_to_string(&path) {
            Ok(content) => Ok(content),
            Err(err) => Err(Box::new(err)),
        }
    } else {
        Err(Box::new(FileExtensionError {
            path: path.to_owned(),
        }))
    }
}
