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
    println!("{:?}", manifest);
    Ok(())
}