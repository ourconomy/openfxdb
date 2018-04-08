# openfxdb
An open, free, and libre database for effects in the economy

It's backend for [ourconomy](https://github.com/ourconomy/ourconomy) and it can handle products and services (called 'effects' in the code). 

The technical base is the amazing project [Open Fair DB](https://github.com/flosse/openfairdb). A lot of functionality that already existed for 'entries" has been implemented for 'effects", too. 

The present version is work in progress. Todos:

  * Clean up code and redundant comments
  * Add fields to effect data structure
  * Implement ratings for effects
  * ...

## Development
The database is written in [Rust](http://rustlang.org/).

Requirements:

- [Rust](https://www.rust-lang.org/) (nightly)
- [SQLite](https://sqlite.org/) 3.x

### Installing Rust & Cargo

If you're using Ubuntu 16.04 LTS you can run

```
sudo apt-get install curl libssl-dev gcc
curl https://sh.rustup.rs -sSf | sh
rustup install nightly
rustup default nightly
```

On windows you can download the installer from [rustup.rs](https://rustup.rs).
(But don't forget to install a
[C++ toolchain](http://landinghub.visualstudio.com/visual-cpp-build-tools) first).

Installing a specific nightly version with `rustup` (e.g. `2018-01-04`) is easy:

```
rustup default nightly-2018-01-04
```

### Installing SQLite & Diesel

On Ubuntu:

```
sudo apt-get install sqlite3 libsqlite3-dev
cargo install diesel_cli --no-default-features --features sqlite
```

### Compile & Run

```
git clone https://github.com/ourconomy/openfxdb
cd openfxdb/
diesel migration run
cargo build
./target/debug/openfairdb
```

If you like NixOS, please go to [Open Fair DB](https://github.com/flosse/openfairdb). There you will find some valuable hints.


## REST API

The current REST API is quite basic and will change within the near future.
The base URL is `http://ourconomy.org/fxapi`.

-  `GET /effects/:ID_1,:ID_2,...,:ID_n`
-  `POST /effects`
-  `PUT /effects/:ID`

#### JSON structures

The structure of an `effect` looks like follows:

```
{
  "id"          : String,
  "version"     : Number,
  "created"     : Number,
  "name"        : String,
  "description" : String,
  "origin"      : String,
  "homepage"    : String,
  "tags"        : [String],
  "license"     : String
}
```

## Logging

    RUST_LOG=debug ./target/debug/openfxdb

If you want to get stacktraces on panics use

    export RUST_BACKTRACE=1

## DB Backups

At the moment the openFXDB does not support online backups.
If you want to backup your DB file, please have a look at this 
[script](https://github.com/flosse/openfairdb/blob/master/scripts/backup-sqlite.sh)
.

# License

Copyright (c) 2015 - 2018 Markus Kohlhase and also to a small extent 2018 Oliver Sendelbach

This project is licensed under the AGPLv3 license.
