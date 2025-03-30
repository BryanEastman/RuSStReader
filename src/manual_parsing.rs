#[derive(Deserialize, Debug)]
pub struct Channel {
    title: Option<String>,
    subtitle: Option<String>,
    updated: Option<String>,
    id: Option<String>,
    entry: Option<Vec<Entry>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Entry {
    author: Option<Author>,
    title: Option<String>,
    id: Option<String>,
    published: Option<String>,
    updated: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Author {
    name: Option<Name>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Name {
    #[serde(rename = "$value")]
    name: Option<String>,
}
