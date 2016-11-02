#[macro_use]
extern crate hyper;
extern crate rustc_serialize;
extern crate clap;

mod config;
mod app;

use config::Config;
use app::Wunderist;

use clap::{Arg, App, SubCommand};

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

    let mut config = match Config::new() {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let app = Wunderist::new(config);

    if matches.subcommand_matches("user").is_some() {
        app.get_user();
    }

    if matches.subcommand_matches("lists").is_some() {
        app.get_lists();
    }

    if matches.subcommand_matches("inbox").is_some() {
        app.get_inbox();
    }

    if let Some(m) = matches.subcommand_matches("add") {
        let name = m.value_of("Task Name").unwrap();
        app.add_task_inbox(name);
    }
}
