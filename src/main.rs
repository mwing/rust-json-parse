extern crate rustc_serialize;
extern crate hyper;

use hyper::*;
use rustc_serialize::json::Json;
use std::io::Read;

fn main() {
    let client = Client::new();
    let mut res = client.get("https://www.k-ruoka.fi/kr-api/recipe/chilinen-perunasalaatti").send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    let data = Json::from_str(&s).unwrap();
    let name = data.as_object().unwrap().get("Name");
    let id = data.as_object().unwrap().get("Id");
    println!("{} {}", id.unwrap(), name.unwrap());
}
