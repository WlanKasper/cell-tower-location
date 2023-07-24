//! Server implementation of swagger_client.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;

use std::collections::HashMap;

use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use swagger_client::{Api, ApiError,
                      DecodeWGS84Response,
                      EncodeWGS84Response
};
use swagger_client::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{

    /// WGS84 decode endpoint
    fn decode_wgs84(&self, context: &C) -> Box<Future<Item=DecodeWGS84Response, Error=ApiError>> {
        let context = context.clone();
        println!("decode_wgs84() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// WGS84 encode endpoint
    fn encode_wgs84(&self, context: &C) -> Box<Future<Item=EncodeWGS84Response, Error=ApiError>> {
        let context = context.clone();
        println!("encode_wgs84() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
