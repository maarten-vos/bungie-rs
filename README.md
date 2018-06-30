# bungie-rs [![Build Status](https://travis-ci.com/inferiormartin/bungie-rs.svg?branch=master)](https://travis-ci.com/inferiormartin/bungie-rs)
*A Rust crate for interacting with the Bungie.net API*

NOTE: This crate is in alpha and highly unstable!

## Features

A direct 1:1 link to the Bungie.net API

## Usage
```Rust
extern crate bungie;

use bungie::BungieClient,

fn main() {
    let bungie = BungieClient::new("<api-key>").with_authentication_token("<oauth-token>");
    let manifest = bungie.destiny2().get_destiny_manifest();
}
```