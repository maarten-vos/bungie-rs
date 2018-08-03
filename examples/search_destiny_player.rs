extern crate bungie;
extern crate dotenv;
extern crate failure;

use bungie::{BungieClient, destiny2::models::*};
use std::env;
use dotenv::dotenv;

fn main() -> Result<(), failure::Error> {
    dotenv()?;
    let bungie = BungieClient::new(env::var("API_KEY")?);
    let user_info_card = bungie.destiny2().search_destiny_player(MembershipType::All, "inferior")?;
    println!("{:?}", user_info_card);
    Ok(())
}