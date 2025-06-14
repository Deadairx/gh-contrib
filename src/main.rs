mod api;
mod model;
mod render;

use std::env;
use dotenv::dotenv;
use crate::model::ContributionDay;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let args: Vec<String> = env::args().collect();
    let username = args.get(1)
        .map(|s| s.to_string())
        .or_else(|| env::var("GH_USER").ok())
        .ok_or("Please provide a GitHub username or set GH_USER environment variable")?;

    let response = api::fetch_contributions(&username)?;
    
    if let Some(user) = response.user {
        let days: Vec<ContributionDay> = user.contributions_collection
            .contribution_calendar
            .weeks
            .into_iter()
            .flat_map(|week| week.contribution_days)
            .map(|day| ContributionDay {
                date: day.date,
                contribution_count: day.contribution_count,
                color: day.color,
            })
            .collect();

        render::render_contributions(&days, true)?;
    } else {
        println!("No contributions found for user: {}", username);
    }

    Ok(())
}
