const TRACE_IDENT_PLACEHOLDER: &str = "\t";

/// Used to keep the state about trace levels
pub struct Tracer {
    trace_level: usize,
}

impl Tracer {
    pub fn new() -> Self {
        Tracer { trace_level: 0 }
    }

    pub fn trace<'a>(&mut self, message: &'a str) -> &'a str {
        self.increment_ident();
        self.trace_print(format!("BEGIN {}", message));
        message
    }

    pub fn un_trace(&mut self, message: &str) {
        self.trace_print(format!("END {}", message));
        self.decrement_ident();
    }

    fn ident_level(&self) -> String {
        TRACE_IDENT_PLACEHOLDER.repeat(self.trace_level - 1)
    }

    fn trace_print(&self, fs: String) {
        if !unsafe { super::TRACING_ENABLED } {
            return;
        }
        println!("{}{}", self.ident_level(), fs)
    }

    fn increment_ident(&mut self) {
        self.trace_level += 1;
    }

    fn decrement_ident(&mut self) {
        self.trace_level -= 1;
    }
}
