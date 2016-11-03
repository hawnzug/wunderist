use config::Config;

use hyper;
use hyper::{Client, Url};
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::io::Read;
use std::fmt;

header! { (XAccessToken, "X-Access-Token") => [String] }
header! { (XClientID, "X-Client-ID") => [String] }

pub struct Wunderist {
    cfg: Config,
}

impl Wunderist {
    pub fn new(config: Config) -> Wunderist {
        Wunderist { cfg: config }
    }

    pub fn get_headers(&self) -> Result<Headers, Error> {
        let mut headers = Headers::new();
        let token = try!(self.cfg
                             .get("X-Access-Token")
                             .ok_or(Error::AccessToken));
        let id = try!(self.cfg
                          .get("X-Client-ID")
                          .ok_or(Error::ClientID));
        headers.set(XClientID(id.to_string()));
        headers.set(XAccessToken(token.to_string()));
        Ok(headers)
    }

    pub fn get_user(&self) -> Result<(), Error> {
        let client = Client::new();
        let url = Url::parse("http://a.wunderlist.com/api/v1/user").unwrap();
        let request = client.get(url).headers(try!(self.get_headers()));
        let mut res = try!(request.send());
        let mut buf = String::with_capacity(65535);
        match res.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(e) => println!("err => {:?}", e),
        }
        let data = try!(Json::from_str(&buf));
        let obj = if let Some(obj) = data.as_object() {
            obj
        } else {
            return Err(Error::JsonData("User data is not an object".to_string()));
        };
        if let Some(&Json::String(ref name)) = obj.get("name") {
            println!("name: {}", name);
        } else {
            return Err(Error::JsonData("no name in user info".to_string()));
        }
        if let Some(&Json::String(ref email)) = obj.get("email") {
            println!("email: {}", email);
        } else {
            return Err(Error::JsonData("no email in user info".to_string()));
        }
        Ok(())
    }

    pub fn get_lists(&self) -> Result<(), Error> {
        let client = Client::new();
        let url = Url::parse("http://a.wunderlist.com/api/v1/lists").unwrap();
        let request = client.get(url).headers(try!(self.get_headers()));
        let mut res = try!(request.send());
        let mut buf = String::with_capacity(65535);
        match res.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(e) => println!("err => {:?}", e),
        }
        let lists = if let Json::Array(lists) = try!(Json::from_str(&buf)) {
            lists
        } else {
            return Err(Error::JsonData("All list info is not an array".to_string()));
        };
        for list in lists {
            let obj = if let Some(obj) = list.as_object() {
                obj
            } else {
                return Err(Error::JsonData("List is not an object".to_string()));
            };
            if let Some(&Json::String(ref title)) = obj.get("title") {
                println!("{}", title);
            } else {
                return Err(Error::JsonData("No list title".to_string()));
            }
        }
        Ok(())
    }

    pub fn get_inbox_id(&self) -> Result<String, Error> {
        if let Some(ref id) = self.cfg.get("inbox-id") {
            return Ok(id.to_string());
        }
        let client = Client::new();
        let url = Url::parse("http://a.wunderlist.com/api/v1/lists").unwrap();
        let request = client.get(url).headers(try!(self.get_headers()));
        let mut res = try!(request.send());
        let mut buf = String::with_capacity(65535);
        match res.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(e) => println!("err => {:?}", e),
        }
        let lists = if let Json::Array(lists) = try!(Json::from_str(&buf)) {
            lists
        } else {
            return Err(Error::JsonData("Todo List is not an array".to_string()));
        };
        for list in lists {
            let obj = if let Some(obj) = list.as_object() {
                obj
            } else {
                return Err(Error::JsonData("List is not an object".to_string()));
            };
            let list_type = if let Some(&Json::String(ref list_type)) = obj.get("list_type") {
                list_type
            } else {
                return Err(Error::JsonData("No list_type in list".to_string()));
            };
            if list_type == "inbox" {
                if let Some(&Json::U64(id)) = obj.get("id") {
                    return Ok(id.to_string());
                } else {
                    return Err(Error::JsonData("No list id in list".to_string()));
                }
            }
        }
        return Err(Error::InboxID);
    }

    pub fn get_inbox(&self) -> Result<(), Error> {
        let id = try!(self.get_inbox_id());
        let client = Client::new();
        let mut url = Url::parse("http://a.wunderlist.com/api/v1/tasks").unwrap();
        url.query_pairs_mut().clear().append_pair("list_id", &id);
        let request = client.get(url).headers(try!(self.get_headers()));
        let mut res = try!(request.send());
        let mut buf = String::with_capacity(65535);
        match res.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(e) => println!("err => {:?}", e),
        }
        let list = if let Json::Array(list) = try!(Json::from_str(&buf)) {
            list
        } else {
            return Err(Error::JsonData("Todo list is not an array".to_string()));
        };
        for item in list {
            let obj = if let Some(obj) = item.as_object() {
                obj
            } else {
                return Err(Error::JsonData("Todo Item is not an object".to_string()));
            };
            if let Some(&Json::String(ref title)) = obj.get("title") {
                println!("{}", title);
            } else {
                return Err(Error::JsonData("No title in todo item".to_string()));
            }
        }
        Ok(())
    }

    pub fn add_task_inbox(&self, name: &str) -> Result<(), Error> {
        let client = Client::new();
        let url = Url::parse("http://a.wunderlist.com/api/v1/tasks").unwrap();
        let data = format!("{{\"list_id\": 195185253, \"title\": \"{}\"}}", name);
        let mut headers = try!(self.get_headers());
        headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));
        let request = client.post(url).headers(headers).body(&data);
        let mut res = try!(request.send());
        let mut buf = String::with_capacity(65535);
        match res.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(e) => println!("err => {:?}", e),
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    AccessToken,
    ClientID,
    InboxID,
    JsonData(String),
    ParseJson(json::ParserError),
    Http(hyper::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AccessToken => write!(f, "Missing Access Token in config"),
            Error::ClientID => write!(f, "Missing Client ID in config"),
            Error::InboxID => write!(f, "Something wrong with Inbox ID"),
            Error::JsonData(ref s) => write!(f, "{}", s),
            Error::Http(ref err) => err.fmt(f),
            Error::ParseJson(ref err) => err.fmt(f),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            AccessToken => "No Access Token in config",
            ClientID => "No Client ID in config",
            InboxID => "Something wrong with Inbox ID",
            JsonData(ref s) => &s,
            Http(ref err) => err.description(),
            ParseJson(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        use self::Error::*;
        match *self {
            AccessToken | ClientID | InboxID | JsonData(_) => None,
            Http(ref err) => Some(err),
            ParseJson(ref err) => Some(err),
        }
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        Error::Http(err)
    }
}

impl From<json::ParserError> for Error {
    fn from(err: json::ParserError) -> Error {
        Error::ParseJson(err)
    }
}
