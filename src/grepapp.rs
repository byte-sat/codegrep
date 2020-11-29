use {anyhow::Result, reqwest::Url, serde::Deserialize};

#[derive(Debug, Clone)]
pub struct GrepApp {
    search_url: Url,
}

impl GrepApp {
    pub fn new() -> Self {
        GrepApp {
            search_url: Url::parse("https://grep.app/api/search?format=e").unwrap(),
        }
    }

    pub async fn search(&self, params: &SearchParams) -> Result<SearchResult> {
        let mut url = self.search_url.clone();
        url.query_pairs_mut().append_pair("q", &params.query);
        if params.page > 1 {
            url.query_pairs_mut()
                .append_pair("page", &params.page.to_string());
        }
        if params.case_sensitive {
            url.query_pairs_mut().append_pair("case", "true");
        }
        if params.regex {
            url.query_pairs_mut().append_pair("regexp", "true");
        }
        if params.words {
            url.query_pairs_mut().append_pair("words", "true");
        }
        for lang in &params.langs {
            url.query_pairs_mut().append_pair("f.lang", &lang);
        }
        if let Some(repo) = &params.repo {
            url.query_pairs_mut().append_pair("f.repo.pattern", &repo);
        }
        if let Some(path) = &params.path {
            url.query_pairs_mut().append_pair("f.path.pattern", &path);
        }
        Ok(reqwest::get(url).await?.json::<SearchResult>().await?)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SearchParams {
    pub page: i64,
    pub query: String,
    pub case_sensitive: bool,
    pub regex: bool,
    pub words: bool,
    pub langs: Vec<String>,
    pub repo: Option<String>,
    pub path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SearchResult {
    pub facets: Facets,
    pub hits: Hits,
    pub partial: bool,
    pub time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Facets {
    pub count: i64,
    #[serde(rename = "lang", default, deserialize_with = "de_buckets")]
    pub langs: Vec<Bucket>,
    #[serde(rename = "path", default, deserialize_with = "de_buckets")]
    pub paths: Vec<Bucket>,
    #[serde(rename = "repo", default, deserialize_with = "de_buckets")]
    pub repos: Vec<Bucket>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Bucket {
    pub count: i64,
    pub val: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Hits {
    pub hits: Vec<Hit>,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Hit {
    #[serde(deserialize_with = "de_raw_string")]
    pub path: String,
    #[serde(deserialize_with = "de_raw_string")]
    pub repo: String,
    pub content: Content,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Content {
    pub snippet: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ID {
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OwnerID {
    pub raw: String,
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TotalMatches {
    pub raw: String,
}

fn de_raw_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Raw {
        raw: String,
    }

    Raw::deserialize(deserializer).map(|r| r.raw)
}

fn de_buckets<'de, D>(deserializer: D) -> Result<Vec<Bucket>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Buckets {
        buckets: Vec<Bucket>,
    }

    Buckets::deserialize(deserializer).map(|b| b.buckets)
}
