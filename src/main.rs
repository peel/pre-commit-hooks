use github_rs::client::{Executor, Github};
use github_rs::{HeaderMap, StatusCode};
use serde_json::Value;
use std::env;
use clap::{App, Arg};

fn main() {
    let matches = App::new("GitHub Issue Rule")
        .version("1.0")
        .about("Align commit name with GitHub Issue")
        .arg(
            Arg::with_name("owner")
                .required(true)
                .help("Repository owner"),
        )
        .arg(
            Arg::with_name("repo")
                .required(true)
                .help("Repository name"),
        )
        .arg(
            Arg::with_name("token")
                .required(false)
                .help("GitHub user token"),
        )
        .get_matches();
    
    let gh_token_key = "GITHUB_PERSONAL_ACCESS_TOKEN";
    let env_t = env::var(gh_token_key).unwrap();

    let t = matches.value_of("token").or(Some(&env_t)).expect("No token set");
    let client = Github::new(t).expect("failed to create client");
    let o = matches.value_of("owner").expect("No owner set");
    let r = matches.value_of("repo").expect("No repo set");
    
    run(&client, o, r);
}


fn run(client: &Github, owner: &str, repo_name: &str) -> () {
    let issues = get_issues(&client, owner, repo_name).expect("failed to get issues");

    let issues_arr = issues
        .as_array()
        .expect("failed to cast issues to json array");
    
    issues_arr
        .into_iter()
        .map(|x| x.get("title"))
        .filter(|x| x.unwrap() == "Scala Stream Collector: bump to 0.17.0")
        .for_each(|x| println!("{}", x.unwrap()));
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
