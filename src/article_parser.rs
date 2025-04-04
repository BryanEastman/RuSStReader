use bytes::Bytes;
use feed_rs::parser;
use html2text::from_read;
use html_entities::decode_html_entities;
use reqwest::blocking::Client;
use std::error::Error;
use std::fmt;
use std::io::BufReader;

#[derive(Debug)]
pub struct Channel {
    pub title: String,
    updated: String,
    pub articles: Vec<Article>,
}

#[derive(Debug)]
pub struct Article {
    title: String,
    authors: Vec<String>,
    summary: String,
    links: Vec<String>,
    updated: String,
    content: String,
}

pub fn fetch_feeds(client: &Client, url: &String) -> Result<Bytes, Box<dyn Error>> {
    let response = client
        .get(url)
        .send()
        .expect("Request Failed")
        .bytes()
        .unwrap();

    Ok(response)
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n{}\n {}:\n\t{}\n\t{}",
            decode_html_entities(self.title.as_str()).unwrap(),
            self.links.join("; "),
            decode_html_entities(self.summary.as_str()).unwrap(),
            decode_html_entities(
                html2text::from_read(self.content.as_bytes(), 80)
                    .unwrap()
                    .as_str()
            )
            .unwrap()
        )
    }
}

pub fn parse_articles(
    url_list: &Vec<String>,
    client: &Client,
) -> Result<Vec<Channel>, Box<dyn Error>> {
    let mut coll: Vec<Channel> = vec![];

    for url in url_list {
        let feed = fetch_feeds(&client, url).unwrap();
        let br = BufReader::new(feed.as_ref());
        let parsed = parser::parse(br).unwrap();

        let mut new_channel = Channel {
            title: parsed.title.unwrap().content,
            updated: match parsed.updated {
                Some(_) => parsed.updated.unwrap().to_string(),
                _ => "".to_string(),
            },
            articles: vec![],
        };

        for item in parsed.entries {
            let article = Article {
                title: item.title.unwrap().content,
                authors: item.authors.into_iter().map(|author| author.name).collect(),
                summary: item.summary.unwrap().content,
                links: item.links.into_iter().map(|link| link.href).collect(),
                updated: item.updated.unwrap().to_string(),
                content: item.content.unwrap().body.unwrap(),
            };
            new_channel.articles.push(article);
        }
        coll.push(new_channel)
    }

    Ok(coll)
}
