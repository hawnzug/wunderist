use config::Config;

use hyper::{Client, Url};
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use rustc_serialize::json::Json;
use std::io::Read;

header! { (XAccessToken, "X-Access-Token") => [String] }
header! { (XClientID, "X-Client-ID") => [String] }

pub struct Wunderist {
    cfg: Config,
}

impl Wunderist {
    pub fn new(config: Config) -> Wunderist {
        Wunderist {
            cfg: config,
        }
    }

    pub fn get_headers(&self) -> Result<Headers, String> {
        let mut headers = Headers::new();
        let token = try!(self.cfg.cfg.get("X-Access-Token")
                         .ok_or("No X-Access-Token in config!".to_string()));
        let id = try!(self.cfg.cfg.get("X-Client-ID")
                      .ok_or("No X-Client-ID in config!".to_string()));
        headers.set(XClientID(id.to_string()));
        headers.set(XAccessToken(token.to_string()));
        Ok(headers)
    }

    pub fn get_user(&self) {
        let client = Client::new();
        let url = Url::parse("http://a.wunderlist.com/api/v1/user").unwrap();
        let request = client.get(url).headers(self.get_headers().unwrap());
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

    pub fn get_lists(&self) {
        let client = Client::new();
        let url = Url::parse("http://a.wunderlist.com/api/v1/lists").unwrap();
        let request = client.get(url).headers(self.get_headers().unwrap());
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

    pub fn get_inbox_id(&self) -> u64 {
        if let Some(ref id) = self.cfg.cfg.get("inbox-id") {
            return id.parse().unwrap();
        }
        let client = Client::new();
        let url = Url::parse("http://a.wunderlist.com/api/v1/lists").unwrap();
        let request = client.get(url).headers(self.get_headers().unwrap());
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

    pub fn get_inbox(&self) {
        let id = self.get_inbox_id().to_string();
        let client = Client::new();
        let mut url = Url::parse("http://a.wunderlist.com/api/v1/tasks").unwrap();
        url.query_pairs_mut().clear().append_pair("list_id", &id);
        let request = client.get(url).headers(self.get_headers().unwrap());
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

    pub fn add_task_inbox(&self, name: &str) {
        let client = Client::new();
        let url = Url::parse("http://a.wunderlist.com/api/v1/tasks").unwrap();
        let data = format!("{{\"list_id\": 195185253, \"title\": \"{}\"}}", name);
        let mut headers = self.get_headers().unwrap();
        headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));
        let request = client.post(url).headers(headers).body(&data);
        let mut res = request.send().unwrap();
        let mut buf = String::with_capacity(65535);
        match res.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(e) => println!("err => {:?}", e),
        }
    }
}
