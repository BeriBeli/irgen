#[derive(Default)]
pub(crate) struct Writer {
    output: String,
    indent: usize,
}

impl Writer {
    pub(crate) fn line(&mut self, line: impl AsRef<str>) {
        for _ in 0..self.indent {
            self.output.push_str("  ");
        }
        self.output.push_str(line.as_ref());
        self.output.push('\n');
    }

    pub(crate) fn raw_block(&mut self, value: &str) {
        self.output.push_str(value.trim_end());
        self.output.push('\n');
    }

    pub(crate) fn raw_indented_block(&mut self, value: &str) {
        for line in value.lines() {
            self.line(line);
        }
    }

    pub(crate) fn indent(&mut self) {
        self.indent += 1;
    }

    pub(crate) fn dedent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }

    pub(crate) fn blank_line(&mut self) {
        if !self.output.ends_with("\n\n") {
            self.output.push('\n');
        }
    }

    pub(crate) fn finish(mut self) -> String {
        while self.output.ends_with("\n\n") {
            self.output.pop();
        }
        self.output
    }
}
