use scraper::Html;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use scraper::html::Select;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::io::prelude::*;
use std::io::Read;
use reqwest::redirect::Attempt;
use std::collections::HashSet;
use reqwest::Url;
use std::hash::Hash;
use std::time::Instant;
use std::path::Path;

//  -> Result<(), Box<dyn std::error::Error>>
// #[derive(Savefile)]
// #[tokio::main]
pub fn get_product_info() {
    println!("Starting loop");
    save_initial_html();
    // save_component_names_html();
    // scrap_html_from_newegg();
    println!("Ending loop.");
}

fn has_extension(url: &&str) -> bool {
    Path::new(url).extension().is_none()
}

fn fetch_url(client: &reqwest::blocking::Client, url: &str) -> String {
    let mut res = client.get(url).send().unwrap();
    println!("Status for {}: {}", url, res.status());

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

fn get_links_from_html(html: &str) -> HashSet<String> {
    Document::from(html)
        .find(Name("a").or(Name("link")))
        .filter_map(|n| n.attr("href"))
        .filter(has_extension)
        .filter_map(normalize_url)
        .collect::<HashSet<String>>()
}

fn normalize_url(url: &str) -> Option<String> {
    let new_url = Url::parse(url);
    match new_url {
        Ok(new_url) => {
            if let Some("newegg.com") = new_url.host_str() {
                Some(url.to_string())
            } else {
                let mut new_url = url.to_string();
                new_url = new_url[5..].to_string();
                new_url = format!("http{}", new_url);
                println!("{}", new_url);
                Some(new_url.to_string())
            }
        },
        Err(_e) => {
            // Relative urls are not parsed by Reqwest
            if url.starts_with('/') {
                Some(format!("https:{}", url))
            } else {
                None
            }
        }
    }
}

// #[tokio::main]
fn save_initial_html() {
    let now = Instant::now();

    let client = reqwest::blocking::Client::new();
    let origin_url = "https://www.newegg.com/Components/Store/ID-1";

    let body = fetch_url(&client, origin_url);

    let mut visited = HashSet::new();
    visited.insert(origin_url.to_string());
    let found_urls = get_links_from_html(&body);
    let mut new_urls = found_urls
        .difference(&visited)
        .map(|x| x.to_string())
        .collect::<HashSet<String>>();

    while !new_urls.is_empty() {
        let mut found_urls: HashSet<String> = new_urls.iter().map(|url| {
            let body = fetch_url(&client, url);
            let links = get_links_from_html(&body);
            println!("Visited: {} found {} links", url, links.len());
            links
        }).fold(HashSet::new(), |mut acc, x| {
            acc.extend(x);
            acc
        });
        visited.extend(new_urls);

        new_urls = found_urls
            .difference(&visited)
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();
        println!("New URLs: {}", new_urls.len())
    }

    println!("URLs: {:#?}", found_urls);
    println!("{}", now.elapsed().as_secs());

    // let html = Html::parse_document(&resp);
    // if !path_exists(r#"E:\\workspace\\rust\\newegg_scraper\\src\\newegg.html"#) {
    //     let mut f = File::create(r#"E:\\workspace\\rust\\newegg_scraper\\src\\newegg.html"#).unwrap();
    //     println!("Created file");
    //
    //     f.write_all(resp.as_bytes());
    //
    //     f.sync_all().unwrap();
    // }
}

#[tokio::main]
async fn save_component_names_html() {
    // let component_link = reqwest::get(link)
    //     .await
    //     .unwrap()
    //     .text()
    //     .await
    //     .unwrap();

    let document = Document::from(include_str!(r#"E:\\workspace\\rust\\newegg_scraper\\src\\newegg.html"#));

    for node in document.find(Attr("class", "filter-box-label main-nav-third-title")) {
        println!("{} ({:?})", node.text(), node.attr("href").unwrap());
        let link = node.attr("href").unwrap();
        println!("{}", link);
        let resp = reqwest::get(link)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let mut formatted_path = format!("E:\\workspace\\rust\\newegg_scraper\\src\\{}.html", node.text());
        formatted_path = formatted_path.replace(" ", "");
        formatted_path = formatted_path.replace("/", "");
        formatted_path = formatted_path.replace("&", "");
        formatted_path = formatted_path.replace("(NAS)", "");
        formatted_path = formatted_path.replace("-", "");
        println!("{}", formatted_path.to_string());

        // let html = Html::parse_document(&resp);
        let mut f = File::create(formatted_path.to_string()).unwrap();
        println!("Created file");

        f.write_all(resp.as_bytes());

        f.sync_all().unwrap();
    }
}

#[tokio::main]
async fn scrap_html_from_newegg() {
    let pages = ["AdaptersGenderChangers.html", "BackupDevicesMedia.html", "BareboneMiniComputers.html",
    "Cables.html", "CDDVDBluRayBurnersMedia.html", "ComputerAccessories.html",
    "ComputerCases.html", "CPUsProcessors.html", "FansPCCooling.html", "HardDrives.html",
    "Memory.html", "Motherboards.html", "NetworkAttachedStorage.html", "PowerProtection.html",
    "PowerSupplies.html", "ServerComponents.html", "SoundCards.html", "SSDs.html", "USBFlashDrivesMemoryCards.html",
    "VideoCardsVideoDevices.html"];

    let paths = fs::read_dir("./src").unwrap();

    for path in paths {
        let mut html_path = path.unwrap().path().display().to_string().clone();

        if html_path.contains(".html") {
            println!("{}", html_path);

            html_path = html_path.replace("./src\\", "");

            let mut raw_html_path = format!(r#"{}"#, html_path.trim_start());
            raw_html_path = format!(r#"{}"#, html_path.trim_end());

            println!("This is the html after the replace function {}", raw_html_path);

            for i in pages.iter() {
                // let mut file_that_needs_opened = File::open().unwrap();
                // let mut contents = String::new();
                // file_that_needs_opened.read_to_string(&mut contents).unwrap();

                let formatted_page = format!("E:\\workspace\\rust\\newegg_scraper\\src\\{}", i);
                println!("{}", formatted_page);

                let contents = fs::read_to_string(formatted_page).unwrap();
                // println!("{}", contents);

                let document = Document::from_read(contents.as_bytes()).expect("F");
                println!("Got Document");

                for node in document.find(Attr("class", "filter-box-label")) {
                    println!("Started node loop");
                    println!("{} ({:?})", node.text(), node.attr("href").unwrap());
                    let mut link = node.attr("href").expect("Y");
                    println!("{}", link);
                    let formatted_link: String = if !link.contains("https:") {
                        format!("https:{}", link)
                    } else {
                        link.to_owned()
                    };
                    if link.contains("SubCategory") {
                        let resp = reqwest::get(formatted_link)
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap();

                        println!("{}", node.text());
                        let mut path_string = format!("E:\\workspace\\rust\\newegg_scraper\\src\\{}.html", node.text());
                        path_string = path_string.replace(" ", "");
                        path_string = path_string.replace("/", "");
                        path_string = path_string.replace("//", "");
                        println!("{}", path_string.to_string());

                        // let html = Html::parse_document(&resp);
                        let mut f = File::create(path_string.to_string()).unwrap();
                        println!("Created file");

                        f.write_all(resp.as_bytes());

                        f.sync_all().unwrap();
                    }
                }
            }
        }
    }
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
