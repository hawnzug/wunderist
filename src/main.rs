#[macro_use]
extern crate hyper;
extern crate rustc_serialize;
extern crate clap;

use hyper::{Client, Url};
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use clap::{Arg, App, SubCommand};
use rustc_serialize::json::Json;
use std::io::Read;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

header! { (XAccessToken, "X-Access-Token") => [String] }
header! { (XClientID, "X-Client-ID") => [String] }

fn get_headers() -> Headers {
    let path = Path::new(".wunderist");
    let f = File::open(&path).unwrap();
    let file = BufReader::new(&f);
    let mut lines = file.lines();
    let mut headers = Headers::new();
    headers.set(XClientID(lines.next().unwrap().unwrap()));
    headers.set(XAccessToken(lines.next().unwrap().unwrap()));
    headers
}

fn get_user() {
    let client = Client::new();
    let url = Url::parse("http://a.wunderlist.com/api/v1/user").unwrap();
    let request = client.get(url).headers(get_headers());
    let mut res = request.send().unwrap();
    let mut buf = String::with_capacity(65535);
    match res.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(e) => println!("err => {:?}", e),
    }
    let data = Json::from_str(&buf).unwrap();
    let obj = data.as_object().unwrap();
    if let &Json::String(ref name) = obj.get("name").unwrap() {
        println!("name: {}", name);
    }
    if let &Json::String(ref email) = obj.get("email").unwrap() {
        println!("email: {}", email);
    }
}

fn get_lists() {
    let client = Client::new();
    let url = Url::parse("http://a.wunderlist.com/api/v1/lists").unwrap();
    let request = client.get(url).headers(get_headers());
    let mut res = request.send().unwrap();
    let mut buf = String::with_capacity(65535);
    match res.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(e) => println!("err => {:?}", e),
    }
    if let Json::Array(lists) = Json::from_str(&buf).unwrap() {
        for list in lists {
            let obj = list.as_object().unwrap();
            if let &Json::String(ref title) = obj.get("title").unwrap() {
                println!("title: {}", title);
            }
            if let &Json::U64(id) = obj.get("id").unwrap() {
                println!("id: {}", id);
            }
        }
    }
}

fn get_inbox_id() -> u64 {
    return 195185253; // TODO: add this to config file
    let client = Client::new();
    let url = Url::parse("http://a.wunderlist.com/api/v1/lists").unwrap();
    let request = client.get(url).headers(get_headers());
    let mut res = request.send().unwrap();
    let mut buf = String::with_capacity(65535);
    match res.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(e) => println!("err => {:?}", e),
    }
    if let Json::Array(lists) = Json::from_str(&buf).unwrap() {
        for list in lists {
            let obj = list.as_object().unwrap();
            if let &Json::String(ref list_type) = obj.get("list_type").unwrap() {
                if list_type == "inbox" {
                    if let &Json::U64(id) = obj.get("id").unwrap() {
                        return id;
                    }
                }
            }
        }
    }
    return 0;
}

fn get_inbox() {
    let id = get_inbox_id().to_string();
    let client = Client::new();
    let mut url = Url::parse("http://a.wunderlist.com/api/v1/tasks").unwrap();
    url.query_pairs_mut().clear().append_pair("list_id", &id);
    let request = client.get(url).headers(get_headers());
    let mut res = request.send().unwrap();
    let mut buf = String::with_capacity(65535);
    match res.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(e) => println!("err => {:?}", e),
    }
    if let Json::Array(lists) = Json::from_str(&buf).unwrap() {
        for list in lists {
            let obj = list.as_object().unwrap();
            if let &Json::String(ref title) = obj.get("title").unwrap() {
                println!("{}", title);
            }
        }
    }
}

fn add_task_inbox(name: &str) {
    let client = Client::new();
    let url = Url::parse("http://a.wunderlist.com/api/v1/tasks").unwrap();
    let data = format!("{{\"list_id\": 195185253, \"title\": \"{}\"}}", name);
    let mut headers = get_headers();
    headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));
    let request = client.post(url).headers(headers).body(&data);
    let mut res = request.send().unwrap();
    let mut buf = String::with_capacity(65535);
    match res.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(e) => println!("err => {:?}", e),
    }
}

fn main() {
    let matches = App::new("WundeRist")
                      .version("0.1.0")
                      .subcommand(SubCommand::with_name("user").about("print user message"))
                      .subcommand(SubCommand::with_name("lists").about("print all lists"))
                      .subcommand(SubCommand::with_name("inbox").about("print all tasks in inbox"))
                      .subcommand(SubCommand::with_name("add")
                                      .about("add task to inbox")
                                      .arg(Arg::with_name("Task Name")
                                               .help("task name")
                                               .required(true)
                                               .index(1)))
                      .get_matches();

    if matches.subcommand_matches("user").is_some() {
        get_user();
    }

    if matches.subcommand_matches("lists").is_some() {
        get_lists();
    }

    if matches.subcommand_matches("inbox").is_some() {
        get_inbox();
    }

    if let Some(m) = matches.subcommand_matches("add") {
        let name = m.value_of("Task Name").unwrap();
        add_task_inbox(name);
    }
}
