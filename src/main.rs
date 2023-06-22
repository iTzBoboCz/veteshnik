use reqwest::Url;
use rusty_money::iso::{self, Currency};
use scraper::{Html, Selector, node::Text};

#[derive(Debug)]
struct Offer {
    title: String,
    link: Url,
    price: Price,
    is_auction: bool,
    description: String,
    // use rustpostal later
    location: String
}

#[derive(Debug)]
struct Price(PriceTypes);

#[derive(Debug)]
enum PriceTypes {
    Value((u32, Currency)),
    // not published or hidden
    Hidden,
    Unsupported(String)
}

fn get_hacker_news_data() -> Result<String, Box<dyn std::error::Error>> {
    let hn_txt = reqwest::blocking::get("https://www.bazos.cz/search.php?hledat=es-335&rubriky=www&hlokalita=&humkreis=25&cenaod=&cenado=&Submit=Hledat&kitx=ano")?
        .text()?;

    Ok(hn_txt)
}

fn main() {
    let hn_txt = get_hacker_news_data().unwrap();

    let document = Html::parse_document(&hn_txt);

    let offers = Selector::parse("div.inzeraty").unwrap();

    for raw_offer in document.select(&offers) {
        let offer = Html::parse_fragment(&raw_offer.html());

        let title_selector = Selector::parse(".nadpis > a").unwrap();

        let Some((title, link)) = offer.select(&title_selector).into_iter().map(|element | {
            let link = element.value().attr("href")?;
            let title = element.text().collect::<Vec<_>>().join("");

            Some((title, link))
        })
        .flatten()
        .next() else {
            continue;
        };

        let price_selector = Selector::parse(".inzeratycena").unwrap();

        let Some(price) = offer.select(&price_selector).into_iter().map(|element | {
            let price = element.text().collect::<Vec<_>>().join("").trim().to_owned();

            if price == "Dohodou" {
                return Price(PriceTypes::Hidden);
            }

            // Bazos.cz format: 99 990 Kč
            // Bazos.sk format:  4 349 €
            let price_vec = price.split(' ').collect::<Vec<&str>>();
            let (p, raw_val) = price_vec.split_last().unwrap();

            let Some(currency) = (match *p {
                "Kč" => Some(iso::CZK),
                "€" => Some(iso::EUR),
                _ => None
            }) else {
                return Price(PriceTypes::Unsupported(price));
            };

            let value = raw_val.concat().parse::<u32>().unwrap();

            return Price(PriceTypes::Value((value, *currency)));
        })
        .next() else {
            continue;
        };

        let description_selector = Selector::parse(".popis").unwrap();

        let Some(description) = offer.select(&description_selector).into_iter().map(|element | {
            element.text().collect::<Vec<_>>().join("").trim().to_owned()
        })
        .next() else {
            continue;
        };

        let location_selector = Selector::parse(".inzeratylok").unwrap();

        let Some(location) = offer.select(&location_selector).into_iter().map(|element | {
            element.text().collect::<Vec<_>>().join(", ").trim().to_owned()
        })
        .next() else {
            continue;
        };

        let d = Offer {
            title,
            link: Url::parse(link).unwrap(),
            price,
            is_auction: false,
            description,
            location,
        };

        println!("{:?}", d);
    }

}
