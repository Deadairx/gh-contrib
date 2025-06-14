// TUI rendering logic

use crossterm::style::{Color, SetForegroundColor, ResetColor};
use crossterm::execute;
use std::io::{self, Write};
use crate::model::ContributionDay;

const BLOCK: &str = "█";
const BLOCKS: [&str; 5] = ["░", "▒", "▓", "█", "█"];

pub fn render_contributions(days: &[ContributionDay], use_color: bool) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Iterate through days of the week (0-6, where 0 is Sunday)
    for day_of_week in 0..7 {
        // For each day of the week, iterate through weeks
        for week in 0..(days.len() / 7) {
            let index = week * 7 + day_of_week;
            if index < days.len() {
                let day = &days[index];
                if use_color {
                    let color = parse_color(&day.color);
                    execute!(handle, SetForegroundColor(color))?;
                    write!(handle, "{}", BLOCK)?;
                    execute!(handle, ResetColor)?;
                } else {
                    let block_index = (day.contribution_count as f32 / 10.0).min(4.0) as usize;
                    write!(handle, "{}", BLOCKS[block_index])?;
                }
            }
        }
        writeln!(handle)?;
    }
    Ok(())
}

fn parse_color(color: &str) -> Color {
    match color {
        "#ebedf0" => Color::Rgb { r: 235, g: 237, b: 240 }, // Light gray
        "#9be9a8" => Color::Rgb { r: 155, g: 233, b: 168 }, // Light green
        "#40c463" => Color::Rgb { r: 64, g: 196, b: 99 },   // Medium green
        "#30a14e" => Color::Rgb { r: 48, g: 161, b: 78 },   // Dark green
        "#216e39" => Color::Rgb { r: 33, g: 110, b: 57 },   // Darkest green
        _ => Color::White,
    }
}
