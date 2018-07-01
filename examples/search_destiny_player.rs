extern crate bungie;
extern crate dotenv;
extern crate failure;

use bungie::{BungieClient, models::MembershipType};
use std::env;
use dotenv::dotenv;

fn main() -> Result<(), failure::Error> {
    dotenv()?;
    let bungie = BungieClient::new(env::var("API_KEY")?);
    let user_info_card = bungie.destiny2().search_destiny_player(MembershipType::All, "inferior".to_owned())?;
    println!("{:?}", user_info_card.response);
    Ok(())
}