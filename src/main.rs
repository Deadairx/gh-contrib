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

    // Parse color flag
    let mut palette_kind = render::ColorPaletteKind::Green;
    let mut i = 2;
    while i < args.len() {
        if args[i] == "-c" && i + 1 < args.len() {
            palette_kind = match args[i + 1].as_str() {
                "red" => render::ColorPaletteKind::Red,
                "blue" => render::ColorPaletteKind::Blue,
                _ => render::ColorPaletteKind::Green,
            };
            i += 2;
        } else {
            i += 1;
        }
    }
    let palette = render::ColorPalette::new(palette_kind);

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

        render::render_contributions(&days, true, &palette)?;
    } else {
        println!("No contributions found for user: {}", username);
    }

    Ok(())
}
