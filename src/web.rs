/*
 * Provides useful functions for communication with AOC website
 * Quick and Dirty
 * Currently there is no proper error handling. Maybe something for the future
 */
use std::io::Cursor;

use scraper::{Html, Selector};
use html2text::from_read;
use reqwest;
use reqwest::header::COOKIE;

fn get_day_description_html(day: u32, token: &str) -> Html {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}", 2021, day);
    let cookie = format!("session={}", token);
    let response = client
        .get(&url)
        .header(COOKIE, cookie)
        .send().unwrap();
    let html = response.text().unwrap();
    Html::parse_document(&html)
}

fn get_day_input_raw(day: u32, token: &str) -> String {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", 2021, day);
    let cookie = format!("session={}", token);
    let response = client
        .get(&url)
        .header(COOKIE, cookie)
        .send().unwrap();
    response.text().unwrap()
}

/// Gets description to a puzzle of a day. Will include part 2 if present.
pub fn get_description(day: u32) -> Option<String> {
    let token = get_token();
    let html = get_day_description_html(day, &token);
    let selector = Selector::parse("article.day-desc").unwrap();
    let mut elements = html.select(&selector);
    let first_occurence = elements.next(); 
    if first_occurence.is_none() {
        return None
    }
    let mut text = first_occurence.unwrap().html();
    if let Some(element) = elements.next() {
        text.push_str("\r\n");
        text.push_str(element.html().as_str());
    }
    let curs = Cursor::new(&text);
    Some(from_read(curs, 200))
}

/// Gets the day header
pub fn get_day_header(day: u32) -> Option<String> {
    let token = get_token();
    let html = get_day_description_html(day, &token);
    let selector = Selector::parse("article.day-desc > h2").unwrap();
    let mut elements = html.select(&selector);
    elements.next().map(|t| {
        let html = t.html();
        let test = from_read(Cursor::new(html.as_str()), 200); 
        String::from(test.as_str().trim_end())
    })
}

pub fn get_day_input(day: u32) -> String {
    let token = get_token(); 
    get_day_input_raw(day, &token)
}

fn get_token() -> &'static str {
    ""
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test() {
        let day1_header = get_day_header(1);
        println!("{:?}", day1_header);
        assert!(get_day_header(1).is_some());
    }
}
