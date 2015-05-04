extern crate html5ever;
extern crate hyper;

use std::io::Read;
use std::default::Default;

use hyper::Client;

use html5ever::sink::rcdom::RcDom;
use html5ever::{parse, one_input};

// TODO: Combine HttpError and html5ever's parse error into a single type
fn doc_from_url(url: &str) -> RcDom {
    let mut client = Client::new();
    let mut res = client.get(url).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    parse(one_input(body), Default::default())
}
