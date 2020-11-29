use clap::Clap;
use lazy_static::lazy_static;
use regex::Regex;

use codegrep::{Colors, GrepApp, Opts, SearchParams, SearchResult};

const PAGE_SIZE: i64 = 10;

#[tokio::main]
pub async fn main() {
    let opts = Opts::parse();
    let colors = Colors::get(&opts);

    let ga = GrepApp::new();

    let params = SearchParams {
        page: 1,
        query: opts.query.clone(),
        case_sensitive: !opts.case_insensitive,
        regex: !opts.text && !opts.words,
        words: opts.words,
        langs: opts.lang.clone(),
        repo: opts.repo.clone(),
        path: opts.path.clone(),
    };
    let res = ga.search(&params).await.unwrap();

    let total_pages = (res.facets.count + PAGE_SIZE - 1) / PAGE_SIZE;
    let last_page = if opts.pages > 0 {
        use std::cmp::min;
        min(opts.pages, total_pages)
    } else {
        total_pages
    };

    if opts.show_filters {
        //println!("{:#?}", res);
        let langs: String = res
            .facets
            .langs
            .iter()
            .map(|l| format!("{}: {}", l.val, l.count))
            .collect::<Vec<_>>()
            .join(", ");

        println!("languages: [{}]", langs);

        println!();
        println!("results  repo");
        for repo in res.facets.repos {
            println!("{:>7}  {}", repo.count, repo.val);
        }

        println!();
        println!("results  path");
        for path in res.facets.paths {
            println!("{:>7}  {}", path.count, path.val);
        }

        println!();
        println!("pages: {}", total_pages);
        return;
    }

    if res.hits.hits.len() == 0 {
        return;
    }

    print_results(&opts, &colors, &res);

    use futures::{stream, StreamExt};

    let requests = (2..=last_page).map(|p| {
        let ga = ga.clone();
        let params = SearchParams {
            page: p,
            ..params.clone()
        };
        (ga, params)
    });

    let mut results = stream::iter(requests)
        .map(|(ga, params)| async move { ga.search(&params).await.unwrap() })
        .buffered(5);

    while let Some(res) = results.next().await {
        print_results(&opts, &colors, &res);
    }
}

fn print_results(opts: &Opts, colors: &Colors, res: &SearchResult) {
    lazy_static! {
        static ref ROW_RE: Regex = Regex::new("<tr .*?</tr>").unwrap();
        static ref MARK_RE: Regex = Regex::new("<mark.*?>").unwrap();
        static ref REMOVE_TAGS_RE: Regex = Regex::new("<.*?>").unwrap();
        static ref PARSE_LINE_RE: Regex = Regex::new("(\\d+)(.*)").unwrap();
    }

    for hit in &res.hits.hits {
        println!(
            "{}https://github.com/{}/blob/master/{}{}",
            colors.file,
            hit.repo,
            hit.path,
            Colors::reset(),
        );

        let content = &hit.content.snippet;
        let mut jump = false;
        for m in ROW_RE.find_iter(&content) {
            if jump && opts.context {
                println!("");
            }

            let row = m.as_str();
            jump = row.contains("<div class=\"jump\">");

            let matches = row.contains("</mark>");
            if !opts.context && !matches {
                continue;
            }
            let row = MARK_RE
                .replace_all(&row, &colors.pmatch as &str)
                .replace("</mark>", Colors::reset());
            let row = REMOVE_TAGS_RE.replace_all(&row, "");

            let parts = PARSE_LINE_RE.captures(&row).unwrap();
            let no = parts.get(1).unwrap().as_str();
            let line = parts.get(2).unwrap().as_str();
            let line = htmlescape::decode_html(&line).unwrap();

            if opts.no_line_numbers {
                println!("{}", line);
            } else {
                println!("{}{}{}:{}", colors.line_no, no, Colors::reset(), line);
            }
        }
        println!();
    }
}
