// TUI rendering logic

use crossterm::style::{Color, SetForegroundColor, ResetColor};
use crossterm::execute;
use std::io::{self, Write};
use crate::model::ContributionDay;

const BLOCK: &str = "██";
const BLOCKS: [&str; 5] = ["░", "▒", "▓", "█", "█"];

// GitHub contribution level colors
const NO_CONTRIB: &str = "#ebedf0";     // No contributions
const LIGHT_CONTRIB: &str = "#9be9a8";  // 1-3 contributions
const MEDIUM_CONTRIB: &str = "#40c463"; // 4-6 contributions
const HEAVY_CONTRIB: &str = "#30a14e";  // 7-9 contributions
const MAX_CONTRIB: &str = "#216e39";    // 10+ contributions

const CURRENT_WEEK: usize = 1;

pub fn render_contributions(days: &[ContributionDay], use_color: bool) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Iterate through days of the week (0-6, where 0 is Sunday)
    for day_of_week in 0..7 {
        // For each day of the week, iterate through weeks
        for week in 0..((days.len() / 7) + CURRENT_WEEK) {
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

const DIM: Color = Color::Rgb { r: 0, g: 75, b: 35 };
const NORMAL: Color = Color::Rgb { r: 0, g: 114, b: 0 };
const BRIGHT: Color = Color::Rgb { r: 56, g: 176, b: 0 };
const MAX: Color = Color::Rgb { r: 158, g: 240, b: 26 };

fn parse_color(color: &str) -> Color {
    match color {
        NO_CONTRIB => Color::Rgb { r: 35, g: 37, b: 40 },
        MAX_CONTRIB => MAX,
        HEAVY_CONTRIB => BRIGHT,
        MEDIUM_CONTRIB => NORMAL,
        LIGHT_CONTRIB => DIM,
        _ => Color::Black,
    }
}
