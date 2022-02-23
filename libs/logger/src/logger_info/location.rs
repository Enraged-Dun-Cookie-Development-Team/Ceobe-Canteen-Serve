use std::{borrow::Cow, fmt::Display};

pub struct Location<'re> {
    pub model: &'re str,
    pub file: &'re str,
    pub line: usize,
}

impl<'re> Location<'re> {
    pub(crate) fn new(model: &'re str, path: &'re str, line: u32) -> Self {
        Self {
            model,
            file: path,
            line: line as usize,
        }
    }
}

impl Display for Location<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = if self.file.len() <= 25 {
            Cow::Borrowed(self.file)
        } else {
            let cut = &self.file[self.file.len() - 23..self.file.len()];
            Cow::Owned(format!("..{}", cut))
        };

        write!(f, "[{} | {:<25}:{}]", self.model, file, self.line)
    }
}
