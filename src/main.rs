use reqwest;
use scraper::{self, Html, Selector};

fn main() {
    // println!("Hello, world!");
    let response = get_response();
    let document = parse_doc(&response);

    let unique_html_tag = "h3.lister-item-header>a"; // finds `<a>` tags that have as a parent an `<h3>` tag that is of a `lister-item-header` class

    let title_selector = find_title_selector(unique_html_tag);

    get_title(&document, &title_selector);
}

fn get_response() -> String {
    let response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();

    response
}

fn parse_doc(response: &str) -> Html {
    let doc = scraper::Html::parse_document(&response);
    doc
}

fn find_title_selector(selector: &str) -> Selector {
    let title_selector = scraper::Selector::parse(selector).unwrap();
    title_selector
}

fn get_title(document: &Html, title_selector: &Selector) {
    let titles = document.select(title_selector).map(|x| x.inner_html());

    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{number}, {item}"));
}
