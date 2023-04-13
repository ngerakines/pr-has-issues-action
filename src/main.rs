use anyhow::{Result, Error};
use std::{process::ExitCode, env, fs::File, io::BufReader};

#[tokio::main]
async fn main() -> Result<ExitCode> {

    println!("GITHUB_HEAD_REF {}", env::var("GITHUB_HEAD_REF").unwrap_or_default());
    println!("GITHUB_REPOSITORY {}", env::var("GITHUB_REPOSITORY").unwrap_or_default());
    println!("GITHUB_REF_TYPE {}", env::var("GITHUB_REF_TYPE").unwrap_or_default());
    println!("GITHUB_REPOSITORY_OWNER {}", env::var("GITHUB_REPOSITORY_OWNER").unwrap_or_default());

    let event: serde_json::Value = {
        let event_path = env::var("GITHUB_EVENT_PATH").map_err(|err| Error::msg(err).context("GITHUB_EVENT_PATH not set"))?;
        let file = File::open(event_path)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
    }?;

    println!("{:?}", event);

    // serde_json::from_str("event_path")?;

    // let key_prefixes: Vec<&str> = cli.key_prefixes.split(',').collect();

    // let (owner, repository) = match cli.repository.contains("/") {
    //     true => {
    //         let mut split = cli.repository.split('/');
    //         Ok((split.next().unwrap(), split.next().unwrap()))
    //     },
    //     _ => Err(anyhow!("Invalid repository name")),
    // }?;


    // let client = match cli.github_token {
    //     Some(value) => octocrab::Octocrab::builder()
    //         .personal_token(value.clone())
    //         .build()?,
    //     _ => octocrab::Octocrab::builder().build()?,
    // };

    // let pr = client.pulls(owner, repository).get(cli.pr_num).await;

    // if let Err(err) = pr {
    //     println!("Error: {}", err);
    //     return Ok(ExitCode::FAILURE);
    // }
    // let pr = pr.unwrap();

    // let search_title = pr.title.unwrap_or_default();
    // let search_body = pr.body.unwrap_or_default();

    // let mut title_found = cli.include_title;
    // let mut body_found = cli.include_body;
    // for prefix in key_prefixes.clone() {
    //     if !title_found && search_title.contains(prefix) {
    //         title_found = true;
    //     }
    //     if !body_found && search_body.contains(prefix) {
    //         body_found = true;
    //     }
    //     if title_found && body_found {
    //         break;
    //     }
    // }

    // if !title_found {
    //     println!("Title does not contain any key prefixes");
    //     return Ok(ExitCode::FAILURE);
    // }

    // if !body_found {
    //     println!("Body does not contain any key prefixes");
    //     return Ok(ExitCode::FAILURE);
    // }

    Ok(ExitCode::SUCCESS)
}
