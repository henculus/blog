use crate::error::Error;
use actix_multipart::{Field, Multipart};
use futures::TryStreamExt;

pub async fn read_buffer(multipart: Multipart) -> Result<Vec<u8>, Error> {
    let buffer = multipart
        .map_ok(async move |field| {
            Ok(get_buffer_from_field(field).await?): Result<_, Error>
        })
        .try_fold(vec![], async move |mut vec, buf| {
            vec.append(&mut buf.await.unwrap());
            Ok(vec)
        })
        .await?;
    Ok(buffer)
}

async fn get_buffer_from_field(field: Field) -> Result<Vec<u8>, Error> {
    let buffer = field
        .try_fold(vec![], async move |mut acc, part| {
            acc.extend(part.slice(0..part.len()));
            Ok(acc)
        })
        .await?;
    Ok(buffer)
}
