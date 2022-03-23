use std::fmt::Display;
#[derive(Debug,Clone)]
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
        let model = if self.model.len() < 20 {
            format!("{:->20}", self.model)
        } else {
            let cut = &self.model[self.model.len() - 18..self.model.len()];
            format!("..{}", cut)
        };

        let file = if self.file.len() <= 25 {
            format!("{:->25}", self.file)
        } else {
            let cut = &self.file[self.file.len() - 23..self.file.len()];
            format!("..{}", cut)
        };

        write!(f, "[{} | {}:{:04}]", model, file, self.line)
    }
}
