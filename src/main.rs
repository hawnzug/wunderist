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
use std::collections::HashMap;

header! { (XAccessToken, "X-Access-Token") => [String] }
header! { (XClientID, "X-Client-ID") => [String] }

type Config = HashMap<String, String>;

fn get_config() -> Result<Config, String> {
    let path = Path::new(".wunderist");
    let f = File::open(&path).unwrap();
    let file = BufReader::new(&f);

    let mut config = HashMap::new();
    for (i, line) in file.lines().enumerate() {
        let l = line.unwrap();
        let v: Vec<&str> = l.split(':').map(str::trim).collect();
        if v.len() != 2 {
            return Err(format!("config error: more than one colons at line {}", i + 1));
        }
        config.insert(v[0].to_string(), v[1].to_string());
    }
    Ok(config)
}

fn get_headers(config: &Config) -> Result<Headers, String> {
    let mut headers = Headers::new();
    let token = try!(config.get("X-Access-Token")
                           .ok_or("No X-Access-Token in config!".to_string()));
    let id = try!(config.get("X-Client-ID")
                        .ok_or("No X-Client-ID in config!".to_string()));
    headers.set(XClientID(id.to_string()));
    headers.set(XAccessToken(token.to_string()));
    Ok(headers)
}

fn get_user(config: &Config) {
    let client = Client::new();
    let url = Url::parse("http://a.wunderlist.com/api/v1/user").unwrap();
    let request = client.get(url).headers(get_headers(config).unwrap());
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

fn get_lists(config: &Config) {
    let client = Client::new();
    let url = Url::parse("http://a.wunderlist.com/api/v1/lists").unwrap();
    let request = client.get(url).headers(get_headers(config).unwrap());
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

fn get_inbox_id(config: &Config) -> u64 {
    if let Some(ref id) = config.get("inbox-id") {
        return id.parse().unwrap();
    }
    let client = Client::new();
    let url = Url::parse("http://a.wunderlist.com/api/v1/lists").unwrap();
    let request = client.get(url).headers(get_headers(config).unwrap());
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

fn get_inbox(config: &Config) {
    let id = get_inbox_id(config).to_string();
    let client = Client::new();
    let mut url = Url::parse("http://a.wunderlist.com/api/v1/tasks").unwrap();
    url.query_pairs_mut().clear().append_pair("list_id", &id);
    let request = client.get(url).headers(get_headers(config).unwrap());
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

fn add_task_inbox(name: &str, config: &Config) {
    let client = Client::new();
    let url = Url::parse("http://a.wunderlist.com/api/v1/tasks").unwrap();
    let data = format!("{{\"list_id\": 195185253, \"title\": \"{}\"}}", name);
    let mut headers = get_headers(config).unwrap();
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

    let config = match get_config() {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    if matches.subcommand_matches("user").is_some() {
        get_user(&config);
    }

    if matches.subcommand_matches("lists").is_some() {
        get_lists(&config);
    }

    if matches.subcommand_matches("inbox").is_some() {
        get_inbox(&config);
    }

    if let Some(m) = matches.subcommand_matches("add") {
        let name = m.value_of("Task Name").unwrap();
        add_task_inbox(name, &config);
    }
}
