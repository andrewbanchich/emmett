use crate::inputs::CommonOptions;
use futures::{sync::mpsc::Sender, Async, Poll, Stream};
use serde_json::{json, value::Value};
use std::thread::sleep;
/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-generator.html
use std::time::Duration;

impl<'a> Stream for Generator<'a> {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        sleep(Duration::from_millis(3000));
        let message = json!({ "ip": "108.55.13.247" });
        Ok(Async::Ready(Some(message)))
    }
}

#[derive(Debug)]
pub struct Generator<'a> {
    count: Option<u64>,
    lines: Option<Vec<&'a str>>,
    message: Option<&'a str>,
    threads: Option<u32>,
    _common: CommonOptions<'a>,
    pub _sender: Option<Sender<Value>>,
}

impl<'a> Generator<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl<'a> Default for Generator<'a> {
    fn default() -> Self {
        Self {
            count: Some(0),
            lines: None,
            message: Some("Hello world!"),
            threads: Some(1),
            _common: CommonOptions::default(),
            _sender: None,
        }
    }
}