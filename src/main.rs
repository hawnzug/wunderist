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
                      .subcommand(SubCommand::with_name("user").about("Prints user message"))
                      .subcommand(SubCommand::with_name("lists").about("Prints all lists"))
                      .subcommand(SubCommand::with_name("inbox")
                                      .about("Prints all tasks in inbox"))
                      .subcommand(SubCommand::with_name("config").about("Set configuration"))
                      .subcommand(SubCommand::with_name("add")
                                      .about("Add task to inbox")
                                      .arg(Arg::with_name("Task Name")
                                               .help("task name")
                                               .required(true)
                                               .index(1)))
                      .get_matches();

    if matches.subcommand_matches("config").is_some() {
        let mut config = Config::empty();
        if let Err(err) = config.set_config() {
            println!("{}", err);
            return;
        }
    }

    let config = match Config::new() {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            println!("You can run `wunderist config` to change configuration");
            return;
        }
    };

    let app = Wunderist::new(config);

    if matches.subcommand_matches("user").is_some() {
        match app.get_user() {
            Ok(_) => (),
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
    }

    if matches.subcommand_matches("lists").is_some() {
        match app.get_lists() {
            Ok(_) => (),
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
    }

    if matches.subcommand_matches("inbox").is_some() {
        match app.get_inbox() {
            Ok(_) => (),
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
    }

    if let Some(m) = matches.subcommand_matches("add") {
        let name = m.value_of("Task Name").unwrap();
        match app.add_task_inbox(name) {
            Ok(_) => (),
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
    }
}
