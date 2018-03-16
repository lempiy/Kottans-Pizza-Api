use rusoto_core::Region;
use rusoto_s3::{S3Client, UploadPartRequest};
use std::fs::File;

pub fn configure_s3_client() -> S3Client {
        S3Client::simple(Region::EuCentral1)
}

fn multipart_upload(client: &S3Client, bucket: &str, filename: &str, file: File) {
        let create_multipart_req = CreateMultipartUploadRequest {
                bucket: bucket.to_owned(),
                key: filename.to_owned(),
                ..Default::default()
        };

        // start the multipart upload and note the upload_id generated
        let response = client.create_multipart_upload(&create_multipart_req).sync().expect("Couldn't create multipart upload");

        let upload_id = response.upload_id.unwrap();

        // create 2 upload parts
        let create_upload_part = |body: Vec<u8>, part_number: i64| -> UploadPartRequest {
                UploadPartRequest {
                        body: Some(body),
                        bucket: bucket.to_owned(),
                        key: filename.to_owned(),
                        upload_id: upload_id.to_owned(),
                        part_number: part_number,
                        ..Default::default()
                }
        };

        // minimum size for a non-final multipart upload part is 5MB
        let part = UploadPartRequest {
                body: Some(body),
                bucket: bucket.to_owned(),
                key: filename.to_owned(),
                upload_id: upload_id.to_owned(),
                part_number: part_number,
                ..Default::default()
        };

        // upload parts and note the etags generated for them
        let mut completed_parts = Vec::new();
        for req in [part_req1, part_req2].into_iter() {
                let response = client.upload_part(&req).sync().expect("Couldn't upload a file part");
                println!("{:#?}", response);
                completed_parts.push(CompletedPart {
                        e_tag: response.e_tag.clone(),
                        part_number: Some(req.part_number),
                });
        }

        // complete the multipart upload with the etags of the parts
        let completed_upload = CompletedMultipartUpload { parts: Some(completed_parts) };

        let complete_req = CompleteMultipartUploadRequest {
                bucket: bucket.to_owned(),
                key: filename.to_owned(),
                upload_id: upload_id.to_owned(),
                multipart_upload: Some(completed_upload),
                ..Default::default()
        };

        let response = client.complete_multipart_upload(&complete_req).sync().expect("Couldn't complete multipart upload");
        println!("{:#?}", response);

        // delete the completed file
        test_delete_object(client, bucket, filename);
}
