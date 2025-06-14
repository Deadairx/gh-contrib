// Data struct for API response

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionDay {
    pub date: String,
    pub contribution_count: i64,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Week {
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionCalendar {
    pub total_contributions: i64,
    pub weeks: Vec<Week>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionsCollection {
    pub contribution_calendar: ContributionCalendar,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub contributions_collection: ContributionsCollection,
}

// This type is now only used for rendering
pub type ContributionData = Vec<ContributionDay>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
    pub user: Option<User>,
}
