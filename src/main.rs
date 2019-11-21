use github_rs::client::{Executor, Github};
use github_rs::{HeaderMap, StatusCode};
use serde_json::Value;
use std::env;
use clap::{App, Arg};
use std::process::exit;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

fn main() {
    let matches = App::new("GitHub Issue Rule")
        .version("1.0")
        .about("Align commit name with GitHub Issue")
        .arg(
            Arg::with_name("message-file")
                .help("File containing commit message")
        )
        .arg(
            Arg::with_name("owner")
                .help("Repository owner")
        )
        .arg(
            Arg::with_name("repo")
                .help("Repository name")
        )
        .arg(
            Arg::with_name("token")
                .help("GitHub user token")
        )
        .get_matches();
    
    let gh_token_key = "GITHUB_PERSONAL_ACCESS_TOKEN";
    let env_t = env::var(gh_token_key);

    let m = matches.value_of("message-file").expect("No message file set");
    let o = matches.value_of("owner").expect("No owner set");
    let r = matches.value_of("repo").expect("No repo set");
    let tt = (match env_t {
        Ok(t) => Some(t),
        Err(_) => matches.value_of("token").map(ToString::to_string),
    }).expect("No token set");
    
    let client = Github::new(tt).expect("failed to create client");
    run(&client, m, o, r)
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn run(client: &Github, message_file: &str, owner: &str, repo_name: &str) -> () {
    let issues = get_issues(&client, owner, repo_name).expect("failed to get issues");
    let commit_msg = file_to_vec(message_file).unwrap().pop().unwrap();
    let issues_arr = issues
        .as_array()
        .expect("failed to cast issues to json array");
    
    let is_issue = issues_arr
        .into_iter()
        .map(|x| x.get("title"))
        .any(|x| x.unwrap() == &commit_msg.trim());

    if is_issue {exit(0)} else {exit(1)}
}

fn get_issues(client: &Github, owner: &str, repo_name: &str) -> Option<Value> {
    //endpoint found on https://developer.github.com/v3/issues/#list-issues-for-a-repository
    let issues_endpoint = format!("repos/{}/{}/issues", owner, repo_name);
    //execute
    let response = client
        .get()
        //set custom endpoint here
        .custom_endpoint(&issues_endpoint)
        .execute::<Value>();
    get_json(response)
}

fn get_json(
    response: Result<(HeaderMap, StatusCode, Option<Value>), github_rs::errors::Error>,
) -> Option<Value> {
    match response {
        Ok((_, _, json)) => {
            json
        }
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}
