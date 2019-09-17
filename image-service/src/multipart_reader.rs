use crate::error::Error;
use actix_multipart::{Field, Multipart};
use futures::future::{ok, FutureResult};
use futures::{Future, Stream};

pub fn read_buffer(multipart: Multipart) -> impl Future<Item=Vec<u8>, Error=Error> {
    multipart
        .map(|field| get_buffer_from_field(field).into_stream())
        .flatten()
        .collect()
        .map(|buf| buf.iter().cloned().flatten().collect())
}

fn get_buffer_from_field(field: Field) -> impl Future<Item=Vec<u8>, Error=Error> {
    field
        .from_err()
        .fold(vec![], |mut acc, bytes| {
            acc.extend(bytes);
            ok(acc) as FutureResult<Vec<u8>, Error>
        })
}
