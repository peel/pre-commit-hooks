use github_rs::client::{Executor, Github};
use github_rs::{HeaderMap, StatusCode};
use serde_json::Value;

fn main() {
    let client = Github::new("").expect("failed to create client");
    let owner = "snowplow";
    let repo_name = "snowplow";

    let issues = get_issues(&client, owner, repo_name).expect("failed to get issues");

    let issues_arr = issues
        .as_array()
        .expect("failed to cast issues to json array");
    
    issues_arr
        .into_iter()
        .map(|x| x.get("title"))
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
    print_info_and_get_json(response)
}

//printing headers and status or error and returning json on success
fn print_info_and_get_json(
    response: Result<(HeaderMap, StatusCode, Option<Value>), github_rs::errors::Error>,
) -> Option<Value> {
    match response {
        Ok((headers, status, json)) => {
            println!("{:#?}", headers);
            println!("{}", status);
            json
        }
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}
