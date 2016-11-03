# Wunderist

> A command line tool to play with your Wunderlist, written in Rust.

## Installation

```sh
git clone https://github.com/hawnzug/wunderist
cd wunderist
cargo build --release
```

## Usage
First create new Wunderlist app and acquire your Client ID and Access Token from
[here](https://developer.wunderlist.com/apps/new). You can input arbitrary url
when asked for.

The config is stored in `~/.wunderist`. You can either run `wunderist config`
and enter your Client ID and Access Token, or modify the config file manually.
Put these required things as below.

```
X-Client-ID: **********
X-Access-Token: ********** 
```

Add task to inbox.
```sh
$ wunderist inbox add [name]
```

List all tasks in inbox.
```sh
$ wunderist inbox show
```

Run `wunderist help` for more help.
