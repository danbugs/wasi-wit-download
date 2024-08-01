mod integration_tests;

use clap::{Arg, Command};
use reqwest::get;
use std::{
    fs::{self, File},
    io::{copy, Cursor},
    path::Path,
};
use std::process::exit;
use zip::ZipArchive;
use anyhow::Result;

#[tokio::main]
async fn main() {
    let matches = Command::new("wasi-wit-download")
        .version("1.0")
        .author("danbugs")
        .about("Downloads and extracts specific WIT dependencies from Wasmtime releases")
        .arg(Arg::new("wasmtime-version")
            .help("Wasmtime release version to download")
            .required(true)
            .index(1))
        .arg(Arg::new("folders")
            .help("WIT dependencies to keep")
            .required(true)
            .num_args(1..) // allow multiple arguments WIT deps
            .index(2))
        .get_matches();

    let version = matches.get_one::<String>("wasmtime-version").unwrap();
    let folders: Vec<&String> = matches.get_many::<String>("folders").unwrap().collect();

    println!("Downloading from Wasmtime version: {}", version);
    println!("WIT dependencies to download: {:?}", folders);

    if let Err(e) = download_and_extract_wasmtime_release(version, &folders).await {
        eprintln!("Error: {}", e);
        exit(1);
    }

    exit(0);
}

async fn download_and_extract_wasmtime_release(version: &str, folders: &[&String]) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://github.com/bytecodealliance/wasmtime/archive/refs/tags/v{}.0.0.zip", version);
    let response = get(&url).await?;
    let bytes = response.bytes().await?;
    unzip_and_filter_file(&bytes, folders, version)?;
    Ok(())
}

fn unzip_and_filter_file(bytes: &[u8], folders: &[&String], version: &str) -> Result<()> {
    let reader = Cursor::new(bytes);
    let mut archive = ZipArchive::new(reader)?;

    let folder_prefixes: Vec<String> = folders.iter()
        .map(|folder| format!("wasmtime-{}.0.0/crates/wasi/wit/deps/{}/", version, folder))
        .collect();

    let mut found_folders = vec![];

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let outpath_str = outpath.to_str().unwrap();

        if let Some(prefix) = folder_prefixes.iter().find(|prefix| outpath_str.starts_with(*prefix)) {
            let relative_outpath = outpath.strip_prefix(&format!("wasmtime-{}.0.0/crates/wasi/wit/deps/", version)).unwrap();
            let final_outpath = Path::new(relative_outpath.to_str().unwrap());

            found_folders.push(prefix.clone());

            if file.is_dir() {
                println!("Downloaded: {:?}", final_outpath);
                fs::create_dir_all(&final_outpath)?;
            } else {
                if let Some(parent) = final_outpath.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)?;
                    }
                }
                let mut outfile = File::create(&final_outpath)?;
                copy(&mut file, &mut outfile)?;
            }
        }
    }

    let not_found: Vec<_> = folders.iter()
        .filter(|folder| !found_folders.contains(&format!("wasmtime-{}.0.0/crates/wasi/wit/deps/{}/", version, folder)))
        .collect();

    if !not_found.is_empty() {
        return Err(anyhow::anyhow!("The following folders were not found in the archive: {:?}", not_found));
    }

    Ok(())
}
