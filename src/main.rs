use anyhow::{Error, Result};
use serde::Deserialize;
use std::{env, fs::File, io::BufReader, process::ExitCode};

#[derive(Deserialize)]
struct PullRequest {
    title: Option<String>,
    body: Option<String>,
}

#[derive(Deserialize)]
struct Event {
    number: Option<u64>,
    pull_request: Option<PullRequest>,
}

fn main() -> Result<ExitCode> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <prefixes>", args[0]);
        return Ok(ExitCode::FAILURE);
    }

    let prefixes: Vec<&str> = args[1].split(',').collect();
    if prefixes.is_empty() {
        println!("No prefixes provided");
        return Ok(ExitCode::FAILURE);
    }

    let event: Event = {
        let event_path = env::var("GITHUB_EVENT_PATH")
            .map_err(|err| Error::msg(err).context("GITHUB_EVENT_PATH not set"))?;
        let file = File::open(event_path).map_err(|err| {
            Error::msg(err).context("unable to open file set by GITHUB_EVENT_PATH")
        })?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
    }?;

    if event.number.is_none() {
        println!("PR number not found.");
        return Ok(ExitCode::FAILURE);
    }

    let search_title = event
        .pull_request
        .as_ref()
        .and_then(|pr| pr.title.clone())
        .unwrap_or_default();

    if search_title.is_empty() {
        println!("Title is empty");
        return Ok(ExitCode::FAILURE);
    }

    let search_body = event
        .pull_request
        .and_then(|pr| pr.body)
        .unwrap_or_default();

    if search_body.is_empty() {
        println!("Body is empty");
        return Ok(ExitCode::FAILURE);
    }

    let mut title_found = false;
    let mut body_found = false;
    for prefix in prefixes.clone() {
        if !title_found && search_title.contains(prefix) {
            title_found = true;
        }
        if !body_found && search_body.contains(prefix) {
            body_found = true;
        }
        if title_found && body_found {
            break;
        }
    }

    if !title_found {
        println!("Title does not contain any prefixes");
        return Ok(ExitCode::FAILURE);
    }

    if !body_found {
        println!("Body does not contain any prefixes");
        return Ok(ExitCode::FAILURE);
    }

    Ok(ExitCode::SUCCESS)
}
