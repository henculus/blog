use crate::error::Error;
use actix_multipart::{Field, Multipart};
use futures::{StreamExt, TryStreamExt};

pub async fn read_buffer(multipart: Multipart) -> Result<Vec<u8>, Error> {
//    multipart
//        .map(async move |field| {
//            get_buffer_from_field(field?).await
//        })
//        .
//        .await
}

async fn get_buffer_from_field(field: Field) -> Result<Vec<u8>, Error> {
    field
        .fold(vec![], |mut acc, bytes| {
            acc.extend(bytes);
            Ok(acc)
        })
        .await?
}
