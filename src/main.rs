use reqwest::Url;
use rusty_money::iso::Currency;
use time::OffsetDateTime;

use crate::extractors::{Extract, Page};

// mod scheduler;
mod extractors;

#[derive(Debug)]
enum StockInfo {
  InStock(Option<u32>),
  OutOfStock,
}

#[derive(Debug)]
enum SaleVariant {
  Regular,
  LimitedTimeOffer { end_time: OffsetDateTime },
  PreOrder { release_date: OffsetDateTime },
  Bundle { items: Vec<String> },
}

#[derive(Debug)]
enum Name {
  Full(String),
  FirstLast(String, String),
}

#[derive(Debug)]
struct ContactInfo {
  name: Option<Name>,
  email: Option<String>,
  phone: Option<String>,
}

#[derive(Debug)]
enum BidStatus {
  CurrentBidOnly((Price, Option<ContactInfo>)),
  StartEndBids {
    starting_bid: (Price, Option<ContactInfo>),
    current_highest_bid: (Price, Option<ContactInfo>),
  },
  AllBids(Vec<(Price, Option<ContactInfo>)>),
}

#[derive(Debug)]
struct SaleListing {
  regular_price: Price,
  sale_price: Option<Price>,
  sale_variant: SaleVariant,
}

#[derive(Debug)]
enum ListingType {
  Sale(SaleListing),
  Auction {
    end_time: OffsetDateTime,
    bid_status: BidStatus,
  },
}

#[derive(Debug)]
enum ItemState {
  New,
  Used,
  Refurbished,
  OpenBox,
}

#[derive(Debug)]
struct Offer {
  title: String,
  link: Url,
  listing_type: ListingType,
  description: String,
  // use rustpostal later
  item_state: Option<ItemState>,
  stock_info: Option<StockInfo>,
  location: String,
  sale_ended: bool,
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
