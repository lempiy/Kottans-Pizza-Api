use rusoto_core::Region;
use rusoto_s3::S3Client;

pub fn configure_s3_client() -> S3Client {
        S3Client::simple(Region::EuCentral1)
}

pub fn upload_image() {

}
