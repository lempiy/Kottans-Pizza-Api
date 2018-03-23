use rusoto_core::Region;
use rusoto_s3::{PutObjectError, PutObjectOutput, PutObjectRequest, S3, S3Client};
use std::fs::File;
use std::io::Read;
use std::sync::MutexGuard;

pub fn configure_s3_client() -> S3Client {
    S3Client::simple(Region::EuCentral1)
}

pub fn put_object_with_filename(
    client: &MutexGuard<S3Client>,
    bucket: &str,
    mut f: File,
    dest_filename: &str,
) -> Result<PutObjectOutput, PutObjectError> {
    let mut contents: Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(_) => Err(PutObjectError::Unknown(
            "Cannot provided open file".to_string(),
        )),
        Ok(_) => {
            let req = PutObjectRequest {
                acl: Some("public-read".to_string()),
                bucket: bucket.to_owned(),
                key: ("upload/".to_string() + dest_filename).to_string(),
                body: Some(contents),
                ..Default::default()
            };
            client.put_object(&req).sync()
        }
    }
}
