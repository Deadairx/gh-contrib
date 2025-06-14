// GitHub GraphQL client and query logic
use graphql_client::{GraphQLQuery, Response};
use std::env;
use std::fs;
use reqwest::blocking::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",  // can be empty or dummy
    query_path = "src/graphql/contributions.graphql",
    response_derives = "Debug"
)]
pub struct Contributions;

pub fn fetch_contributions(user: &str) -> Result<contributions::ResponseData, Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN")?;
    let variables = contributions::Variables {
        login: user.to_string(),
    };

    let request_body = Contributions::build_query(variables);
    let client = Client::new();
    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .header("User-Agent", "github-contrib-cli")
        .json(&request_body)
        .send()?;

    let response_body: Response<contributions::ResponseData> = res.json()?;
    response_body
        .data
        .ok_or_else(|| "No data received from GitHub".into())
}
