use reqwest::Url;
use rusty_money::iso::Currency;

use crate::extractors::{Extract, Page};

// mod scheduler;
mod extractors;

#[derive(Debug)]
struct Offer {
  title: String,
  link: Url,
  price: Price,
  is_auction: bool,
  description: String,
  // use rustpostal later
  location: String,
}

#[derive(Debug)]
struct Price(PriceTypes);

#[derive(Debug)]
enum PriceTypes {
  Value((u32, Currency)),
  // not published or hidden
  Hidden,
  Unsupported(String),
}


fn main() {
  // println!("{}", EXTRACTORS.get());
  let url = "https://www.bazos.cz/search.php?hledat=es-335&rubriky=www&hlokalita=&humkreis=25&cenaod=&cenado=&Submit=Hledat&kitx=ano";
  // let bazos = extractors::bazos::Bazos {};
  // println!("{:?}", extractors::bazos::Bazos::name());
  // println!("{:?}", offers);

  let bazos = extractors::Extractors::new();
  // println!("{:?}", bazos.into_inner());
  for x in bazos.into_inner() {
    println!("{:?}", x.take(1));
    // println!("{:?}", x.collect::<Vec<_>>());
  }
}
