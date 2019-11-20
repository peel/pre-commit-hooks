use github_rs::client::{Executor, Github};
use github_rs::{HeaderMap, StatusCode};
use serde_json::Value;
use std::env;

fn main() {
    let gh_token_key = "GITHUB_PERSONAL_ACCESS_TOKEN";
    let err_msg = format!("Github personal access token is needed in {}.", gh_token_key);
    let gh_token = env::var(gh_token_key).expect(&err_msg);
    let client = Github::new(gh_token).expect("failed to create client");
    let owner = "snowplow";
    let repo_name = "snowplow";
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
