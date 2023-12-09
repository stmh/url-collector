use simplelog::*;
use sitemap::{reader::{SiteMapReader,SiteMapEntity}, structs::UrlEntry};
use clap::{Parser};
use anyhow::Result;
use url::Url;
use std::io::Cursor;
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;
use rand::seq::IteratorRandom;
use serde::{Serialize, Deserialize};

type UrlVec  = Vec<UrlEntry>;

#[derive(clap::ValueEnum, Clone, Debug)]
#[clap(rename_all = "kebab_case")]
enum Output {
    Terminal,
    Json,
    Yaml
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    url: String,

    #[arg(long, value_parser = validate_authentication)]
    authentication: Option<String>,

    #[arg(long, default_value_t=50)]
    num_threads: usize,

    #[arg(long, default_value_t=1000)]
    num_urls: usize,

    #[arg(long, value_enum, default_value="terminal")]
    output: Output
}

fn validate_authentication(s: &str) -> Result<String, String> {
    let v: Vec<&str> = s.split(':').collect();
    match v.len() == 2 {
        true => Ok(s.to_owned()),
        false=> Err("Authentication seems to be invalid, use 'user:password' schema".to_owned())
    }
}

fn build_sitemap_url(args: &Args, sitemap_name: &str) -> anyhow::Result<Url> {
    let base_url = Url::parse(&args.url).expect("Failed to parse base URL");

    let mut full_url = base_url.join(sitemap_name).expect("Failed to join URL");

    if let Some(ref s) = &args.authentication {

        let v: Vec<&str> = s.split(':').collect();

        full_url.set_username(v[0]).expect("Failed to set username on base url");
        full_url.set_password(Some(v[1])).expect("Failed to set password on base url");
    }

    Ok(full_url)
}

fn get_sitemap_content(url: Url) -> anyhow::Result<UrlVec> {

    info!("Getting sitemap from {}", url);

    let body = reqwest::blocking::get(url)?.text();

    let mut urls = UrlVec::new();
    let mut errors = Vec::new();
    let mut sitemaps = Vec::new();

    // Create a reader for the response body
    let cursor = Cursor::new(body?);

    // Parse the sitemap
    let parser = SiteMapReader::new(cursor);

    // Iterate through the sitemap entities
    for entity in parser {
        match entity {
            SiteMapEntity::Url(url_entry) => {
                urls.push(url_entry);
            },
            SiteMapEntity::SiteMap(sitemap_entry) => {
                sitemaps.push(sitemap_entry);
            },
            SiteMapEntity::Err(error) => {
                errors.push(error);
            },
        }
    }


    let subsitemaps: Vec<anyhow::Result<UrlVec>> = sitemaps.par_iter()
        .progress_count(sitemaps.len() as u64)
        .filter_map(|sitemap_entry| {
            let results = get_sitemap_content(sitemap_entry.loc.get_url().expect("Sitemap URL expected"));
            Some(results)
        })
        .collect();

    for result in subsitemaps {
        match result {
            Ok(mut found_urls) => urls.append(&mut found_urls),
            Err(e) => {
                error!("{}", e)
            }
        }
    }

    info!("Collected {} urls!", urls.len());
    Ok(urls)
}

#[derive(Serialize, Deserialize)]
struct ResultData {
    num_results: usize,
    urls: Vec<Url>
}

fn output_to_terminal(result: &ResultData) -> Result<()> {
    println!("Collected {} urls", result.num_results);

    for entry in &result.urls {
        println!("{}", entry);
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();


    let args = Args::parse();

    rayon::ThreadPoolBuilder::new().num_threads(args.num_threads).build_global().unwrap();

    info!("Collecting urls from {}!", args.url);
    match args.authentication {
        Some(ref str) => info!("using authentication {}", &str),
        None => info!("using no authentication")
    }

    let sitemap_url = build_sitemap_url(&args, "sitemap.xml");

    let result = get_sitemap_content(sitemap_url.expect("Could not create URL"));

    match result {
        Ok(sitemap_content) => {

            let mut rng = rand::thread_rng();
            let subset: Vec<_> = sitemap_content.iter().choose_multiple(&mut rng, args.num_urls);

            let result_data = ResultData {
                num_results: sitemap_content.len(),
                urls: subset.iter().map(|e| e.loc.get_url().unwrap()).collect()
            };
            match args.output {
                Output::Terminal => output_to_terminal(&result_data),
                Output::Json => output_to_json(&result_data),
                Output::Yaml => output_to_yaml(&result_data),
            }
        },
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    }

}


fn output_to_yaml(result: &ResultData) -> Result<()> {
    // Serialize the data structure into a YAML string.
    let s = serde_yaml::to_string(&result).unwrap();

    println!("{}", s);

    Ok(())
}

fn output_to_json(result: &ResultData) -> Result<()> {
    // Serialize the data structure into a JSON string.
    let s = serde_json::to_string(&result).unwrap();

    println!("{}", s);
    Ok(())
}
