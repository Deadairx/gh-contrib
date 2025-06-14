// GitHub GraphQL client and query logic
use graphql_client::{GraphQLQuery, Response};
use std::env;
use reqwest::blocking::Client;

// Include the generated module
include!(concat!(env!("OUT_DIR"), "/contributions_query.rs"));

pub fn fetch_contributions(user: &str) -> Result<contributions_query::ResponseData, Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN")?;
    let variables = contributions_query::Variables {
        login: user.to_string(),
    };

    let request_body = ContributionsQuery::build_query(variables);
    let client = Client::new();
    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .header("User-Agent", "github-contrib-cli")
        .json(&request_body)
        .send()?;

    let response_body: Response<contributions_query::ResponseData> = res.json()?;
    response_body
        .data
        .ok_or_else(|| "No data received from GitHub".into())
}
