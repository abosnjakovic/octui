use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GraphQLResponse {
    pub data: Data,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Data {
    #[serde(alias = "viewer", alias = "user")]
    pub user: User,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub login: String,
    #[serde(rename = "contributionsCollection")]
    pub contributions_collection: ContributionsCollection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContributionsCollection {
    #[serde(rename = "contributionCalendar")]
    pub contribution_calendar: ContributionCalendar,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContributionCalendar {
    #[serde(rename = "totalContributions")]
    pub total_contributions: u32,
    pub weeks: Vec<Week>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Week {
    #[serde(rename = "contributionDays")]
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContributionDay {
    #[allow(dead_code)]
    #[serde(rename = "contributionCount")]
    pub contribution_count: u32,
    pub date: String,
    #[serde(rename = "contributionLevel")]
    pub contribution_level: ContributionLevel,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContributionLevel {
    None,
    FirstQuartile,
    SecondQuartile,
    ThirdQuartile,
    FourthQuartile,
}

impl ContributionLevel {
    pub fn to_color(self) -> Color {
        match self {
            ContributionLevel::None => Color::Rgb(22, 27, 34),
            ContributionLevel::FirstQuartile => Color::Rgb(14, 68, 41),
            ContributionLevel::SecondQuartile => Color::Rgb(0, 109, 50),
            ContributionLevel::ThirdQuartile => Color::Rgb(38, 166, 65),
            ContributionLevel::FourthQuartile => Color::Rgb(57, 211, 83),
        }
    }
}
