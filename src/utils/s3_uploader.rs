use rusoto_core::{Region, EnvironmentProvider, ProvideAwsCredentials};
use rusoto_core::reactor::{RequestDispatcher};
use rusoto_s3::S3Client;


pub fn configure_s3_client() -> S3Client {
        S3Client::new(RequestDispatcher::default(),
                      EnvironmentProvider, Region::EuCentral1)

}