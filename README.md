# bungie-rs [![Build Status](https://travis-ci.com/inferiormartin/bungie-rs.svg?branch=master)](https://travis-ci.com/inferiormartin/bungie-rs)[![Crates.io](https://img.shields.io/crates/v/bungie.svg)](https://crates.io/crates/bungie)
*A Rust crate for interacting with the Bungie.net API*

NOTE: This crate is in alpha and highly unstable!

## Features

A direct 1:1 link to the Bungie.net API

## Usage
```Rust
extern crate bungie;
extern crate dotenv;
extern crate failure;

use bungie::BungieClient;
use std::env;
use dotenv::dotenv;

fn main() -> Result<(), failure::Error> {
    dotenv()?;
    let bungie = BungieClient::new(env::var("API_KEY")?);
    let manifest = bungie.destiny2().get_destiny_manifest()?;
    println!("{}", manifest.response.version);
    Ok(())
}
```
