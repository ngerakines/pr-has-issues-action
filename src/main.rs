use anyhow::{Result, Error};
use serde_json_path::JsonPath;
use std::{process::ExitCode, env, fs::File, io::BufReader};

#[tokio::main]
async fn main() -> Result<ExitCode> {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <key_prefixes>", args[0]);
        return Ok(ExitCode::FAILURE);
    }

    let key_prefixes: Vec<&str> = args[1].split(',').collect();

    let event: serde_json::Value = {
        let event_path = env::var("GITHUB_EVENT_PATH").map_err(|err| Error::msg(err).context("GITHUB_EVENT_PATH not set"))?;
        let file = File::open(event_path).map_err(|err| Error::msg(err).context("GITHUB_EVENT_PATH not set"))?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
    }?;

    let number_path = JsonPath::parse("$.number")?;
    let number_value = number_path.query(&event).at_most_one()?;

    if number_value.is_none() {
        println!("PR number not found.");
        return Ok(ExitCode::FAILURE);
    }

    let title_path = JsonPath::parse("$.pull_request.title")?;
    let search_title = title_path.query(&event).at_most_one()?.map(|v| v.as_str().unwrap_or_default()).unwrap_or_default();

    if search_title.is_empty() {
        println!("Title is empty");
        return Ok(ExitCode::FAILURE);
    }

    let body_path = JsonPath::parse("$.pull_request.body")?;
    let search_body = body_path.query(&event).at_most_one()?.map(|v| v.as_str().unwrap_or_default()).unwrap_or_default();

    if search_body.is_empty() {
        println!("Body is empty");
        return Ok(ExitCode::FAILURE);
    }

    let mut title_found = false;
    let mut body_found = false;
    for prefix in key_prefixes.clone() {
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
        println!("Title does not contain any key prefixes");
        return Ok(ExitCode::FAILURE);
    }

    if !body_found {
        println!("Body does not contain any key prefixes");
        return Ok(ExitCode::FAILURE);
    }

    Ok(ExitCode::SUCCESS)
}
