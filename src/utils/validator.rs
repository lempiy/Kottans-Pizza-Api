use params::File;
use iron::mime::Mime;
use iron::mime::TopLevel::Image;
use iron::mime::SubLevel::Png;
use validator::ValidationError;
use serde::ser::{Serialize, Serializer, SerializeStruct};

pub struct ValidationFile{
    pub file: File
}

impl Serialize for ValidationFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("File", 4)?;
        state.serialize_field("filename", &self.file.filename)?;
        state.end()
    }
}

pub fn validate_image(f: &ValidationFile) -> Result<(), ValidationError> {
    match f.file.content_type {
        Mime(Image, Png, _) => {
            if f.file.size > 5 << 20 {
                Err(ValidationError::new(("Image is to big, max size is 5 MB")))
            } else {
                Ok(())
            }
        },
        _ => Err(ValidationError::new(("Wrong file MIME type - expected: 'image/png'"))),
    }
}

pub fn validate_pizza_size(size: i64) -> Result<(), ValidationError> {
    match size {
        30i64|45i64|64i64 => Ok(()),
        _ => Err(ValidationError::new(("Wrong size of pizza, only 30, 45 or 60 allowed"))),
    }
}
