#[macro_use]
extern crate hyper;
extern crate rustc_serialize;
extern crate clap;

mod config;
mod app;

use config::Config;
use app::Wunderist;

use clap::{Arg, App, SubCommand, AppSettings};

fn main() {
    let matches = App::new("WundeRist")
                      .version("0.1.0")
                      .setting(AppSettings::SubcommandRequired)
                      .subcommand(SubCommand::with_name("user").about("Prints user message"))
                      .subcommand(SubCommand::with_name("list")
                                      .about("Play with all lists")
                                      .setting(AppSettings::SubcommandRequired)
                                      .subcommand(SubCommand::with_name("show")
                                                      .about("Prints all lists"))
                                      .subcommand(SubCommand::with_name("add")
                                                      .about("Add a list")
                                                      .arg(Arg::with_name("List Name")
                                                               .help("list name")
                                                               .required(true)
                                                               .index(1))))
                      .subcommand(SubCommand::with_name("inbox")
                                      .about("Play with inbox")
                                      .setting(AppSettings::SubcommandRequired)
                                      .subcommand(SubCommand::with_name("show")
                                                      .about("Prints all tasks in inbox"))
                                      .subcommand(SubCommand::with_name("add")
                                                      .about("Add task into inbox")
                                                      .arg(Arg::with_name("Task Name")
                                                               .help("task name")
                                                               .required(true)
                                                               .index(1))))
                      .subcommand(SubCommand::with_name("config").about("Set configuration"))
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

    let mut app = Wunderist::new(config);

    if matches.subcommand_matches("user").is_some() {
        if let Err(err) = app.get_user() {
            println!("{}", err);
        }
    } else if let Some(m) = matches.subcommand_matches("list") {
        if m.subcommand_matches("show").is_some() {
            if let Err(err) = app.get_lists() {
                println!("{}", err);
            }
        } else if let Some(mat) = m.subcommand_matches("add") {
            let name = mat.value_of("List Name").unwrap();
            if let Err(err) = app.add_list(name) {
                println!("{}", err);
            }
        }
    } else if let Some(m) = matches.subcommand_matches("inbox") {
        if m.subcommand_matches("show").is_some() {
            if let Err(err) = app.get_inbox() {
                println!("{}", err);
            }
        } else if let Some(mat) = m.subcommand_matches("add") {
            let name = mat.value_of("Task Name").unwrap();
            if let Err(err) = app.add_task_inbox(name) {
                println!("{}", err);
            }
        }
    }
}
