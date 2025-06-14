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

    for (i, day) in days.iter().enumerate() {
        if i > 0 && i % 7 == 0 {
            writeln!(handle)?;
        }

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
    writeln!(handle)?;
    Ok(())
}

fn parse_color(color: &str) -> Color {
    match color {
        "#ebedf0" => Color::Grey,
        "#9be9a8" => Color::Green,
        "#40c463" => Color::Green,
        "#30a14e" => Color::Green,
        "#216e39" => Color::Green,
        _ => Color::White,
    }
}
