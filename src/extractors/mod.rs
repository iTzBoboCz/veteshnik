use reqwest::Url;

use crate::Offer;

pub mod bazos;

pub trait Extract
where
  Self: Iterator + std::fmt::Debug,
{
  fn name() -> &'static str
  where
    Self: Sized;
  fn is_url_supported(&self, url: String) -> bool;
  // fn parse_site(&self) -> Result<(), ()>;
  // fn save_site() -> Result<(), ()> {
  //   todo!();
  // }
  // // fn try_parse(&self, url: &str) -> Result<Page, ()>;
  // fn request_data(url: &str) -> Result<String, Box<dyn std::error::Error>>;
  // fn search(&self, query: &str) -> Result<Page, ExtractorError>;
  fn new() -> Self
  where
    Self: Sized;
}

/// Ability to search a website
/// Not all extractors should implement search
pub trait Search {
  // page?
  fn search(&self) -> Result<Vec<Offer>, ()>;
}

// Sized explained here: https://github.com/dtolnay/async-trait/issues/165
#[derive(Debug)]
pub struct Extractors(Vec<Box<dyn Extract<Item = Page>>>);

impl Extractors {
  pub fn new() -> Self {
    Self(vec![Box::new(bazos::Bazos)])
  }

  /// Searches given query on all base sites
  pub fn search_all(&self, query: String) -> Result<Vec<()>, ()> {
    // self.0.iter().map(|e| e.search(query)).flatten()
    todo!()
  }

  pub fn search(&self) {}

  pub fn into_inner(self) -> Vec<Box<dyn Extract<Item = Page>>> {
    self.0
  }
}

#[derive(Debug)]
pub struct Page {
  domain: Url,
  page: Option<u32>,
}
