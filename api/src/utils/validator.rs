use iron::mime::Mime;
use iron::mime::TopLevel::Image;
use iron::mime::SubLevel::Png;
use validator::ValidationError;
use std::collections::HashMap;
use std::borrow::Cow;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use multipart::server::SavedFile;
use std::collections::HashSet;
use std::hash::Hash;

pub struct ValidationFile {
    pub file: SavedFile,
}

impl Serialize for ValidationFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("File", 4)?;
        state.serialize_field("filename", &self.file.filename)?;
        state.end()
    }
}

pub fn validate_image(f: &ValidationFile) -> Result<(), ValidationError> {
    match f.file.content_type {
        Mime(Image, Png, _) => if f.file.size > 3 << 20 {
            Err(ValidationError {
                code: Cow::from("wrong_image"),
                message: Some(Cow::from("Image is to big, max size is 3 MB")),
                params: HashMap::new(),
            })
        } else {
            Ok(())
        },
        _ => Err(ValidationError {
            code: Cow::from("wrong_image"),
            message: Some(Cow::from("Wrong file MIME type - expected: 'image/png'")),
            params: HashMap::new(),
        }),
    }
}

pub fn validate_pizza_size(size: i64) -> Result<(), ValidationError> {
    match size {
        30i64 | 45i64 | 60i64 => Ok(()),
        _ => Err(ValidationError {
            code: Cow::from("wrong_size"),
            message: Some(Cow::from("Wrong pizza size - only 30, 45 or 60 allowed")),
            params: HashMap::new(),
        }),
    }
}

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
