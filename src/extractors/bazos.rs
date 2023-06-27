use std::str::FromStr;

use crate::{Offer, Price, PriceTypes};

use super::{Extract, Page};

use reqwest::Url;
use rusty_money::iso;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Bazos;

impl Extract for Bazos {
  fn name() -> &'static str {
    "Bazoš (CZ, SK, PL, AT)"
  }

  fn is_url_supported(&self, url: String) -> bool {
    todo!()
  }

  //   fn parse(url: &str) -> Result<(), ()> {
  //     let hn_txt = Self::request_data(url).unwrap();

  //     let document = Html::parse_document(&hn_txt);

  //     let offers_selector = Selector::parse("div.inzeraty").unwrap();

  //     let mut offers = vec![];

  //     for raw_offer in document.select(&offers_selector) {
  //       let offer = Html::parse_fragment(&raw_offer.html());

  //       let title_selector = Selector::parse(".nadpis > a").unwrap();

  //       let Some((title, link)) = offer.select(&title_selector).into_iter().map(|element | {
  //         let link = element.value().attr("href")?;
  //         let title = element.text().collect::<Vec<_>>().join("");

  //         Some((title, link))
  //       })
  //       .flatten()
  //       .next() else {
  //         continue;
  //       };

  //       let price_selector = Selector::parse(".inzeratycena").unwrap();

  //       let Some(price) = offer.select(&price_selector).into_iter().map(|element | {
  //             let price = element.text().collect::<Vec<_>>().join("").trim().to_owned();

  //             if price == "Dohodou" {
  //                 return Price(PriceTypes::Hidden);
  //             }

  //             // Bazos.cz format: 99 990 Kč
  //             // Bazos.sk format:  4 349 €
  //             let price_vec = price.split(' ').collect::<Vec<&str>>();
  //             let (p, raw_val) = price_vec.split_last().unwrap();

  //             let Some(currency) = (match *p {
  //                 "Kč" => Some(iso::CZK),
  //                 "€" => Some(iso::EUR),
  //                 _ => None
  //             }) else {
  //                 return Price(PriceTypes::Unsupported(price));
  //             };

  //             let value = raw_val.concat().parse::<u32>().unwrap();

  //             return Price(PriceTypes::Value((value, *currency)));
  //         })
  //         .next() else {
  //           continue;
  //         };

  //       let description_selector = Selector::parse(".popis").unwrap();

  //       let Some(description) = offer.select(&description_selector)
  //         .into_iter()
  //         .map(|element | {
  //           element.text().collect::<Vec<_>>().join("").trim().to_owned()
  //         })
  //         .next() else {
  //           continue;
  //         };

  //       let location_selector = Selector::parse(".inzeratylok").unwrap();

  //       let Some(location) = offer.select(&location_selector).into_iter().map(|element | {
  //           element.text().collect::<Vec<_>>().join(", ").trim().to_owned()
  //         })
  //         .next() else {
  //             continue;
  //         };

  //       offers.push(Offer {
  //         title,
  //         link: Url::parse(link).unwrap(),
  //         price,
  //         is_auction: false,
  //         description,
  //         location,
  //       });
  //     }

  //     Ok(())
  //   }

  //   fn request_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
  //     let hn_txt = reqwest::blocking::get(url)?.text()?;

  //     Ok(hn_txt)
  //   }

  fn new() -> Self {
    Self
  }
}

impl Iterator for Bazos {
  type Item = Page;

  fn next(&mut self) -> Option<Self::Item> {
    Some(Page {
      domain: Url::from_str("https://bazos.cz").unwrap(),
      page: None,
    })
  }
}
