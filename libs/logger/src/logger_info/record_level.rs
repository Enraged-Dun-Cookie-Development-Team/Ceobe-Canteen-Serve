use std::fmt::Display;

use log::Level;
use yansi::Color;

pub struct RecordLevel {
    pub color: Color,
    pub level: Level,
}

impl RecordLevel {
    pub(crate) fn no_color(lv: Level) -> Self {
        Self {
            color: Color::Unset,
            level: lv,
        }
    }
}

impl Display for RecordLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let st = format!("[{}]", self.color.paint(self.level));
        match self.color {
            Color::Unset => {
                write!(f, "{:<7}", st)
            }
            _ => {
                write!(f, "{:<16}", st)
            }
        }
    }
}

#[test]
fn etet() {
    let lv = RecordLevel::from(Level::Debug);

    println!("[{:>100}[", lv)
}

impl From<Level> for RecordLevel {
    fn from(lv: Level) -> Self {
        let color = match &lv {
            Level::Error => yansi::Color::Red,
            Level::Warn => yansi::Color::Yellow,
            Level::Info => yansi::Color::Green,
            Level::Debug => yansi::Color::Magenta,
            Level::Trace => yansi::Color::Blue,
        };

        Self { level: lv, color }
    }
}
