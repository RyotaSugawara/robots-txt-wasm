use wasm_bindgen::prelude::*;
use robotstxt::{DefaultMatcher,RobotsParseHandler,parse_robotstxt};

#[wasm_bindgen(getter_with_clone)]
pub struct ExportReport {
    pub valid_lines: Vec<u32>,
    pub invalid_lines: Vec<u32>,
}

#[derive(Default)]
struct RobotsStatsReporter {
    valid_lines: Vec<u32>,
    invalid_lines: Vec<u32>,
}

impl RobotsParseHandler for RobotsStatsReporter {
    fn handle_robots_start(&mut self) {
        let n = 2083 * 8; // max line
        self.valid_lines = Vec::with_capacity(n);
        self.invalid_lines = Vec::with_capacity(n);
    }

    fn handle_robots_end(&mut self) {}

    fn handle_user_agent(&mut self, line_num: u32, _user_agent: &str) {
        self.valid_lines.push(line_num);
    }

    fn handle_allow(&mut self, line_num: u32, _value: &str) {
        self.valid_lines.push(line_num);
    }

    fn handle_disallow(&mut self, line_num: u32, _value: &str) {
        self.valid_lines.push(line_num);
    }

    fn handle_sitemap(&mut self, line_num: u32, _value: &str) {
        self.valid_lines.push(line_num);
    }

    fn handle_unknown_action(&mut self, line_num: u32, _action: &str, _value: &str) {
        self.invalid_lines.push(line_num);
    }
}

#[wasm_bindgen]
pub fn is_allowed(txt: &str, user_agent: &str, url: &str) -> bool {
    let mut matcher = DefaultMatcher::default();
    matcher.one_agent_allowed_by_robots(txt, user_agent, url)
}

#[wasm_bindgen]
pub fn validate(txt: &str) -> ExportReport {
    let mut report = RobotsStatsReporter::default();
    parse_robotstxt(txt, &mut report);
    ExportReport {
        valid_lines: report.valid_lines,
        invalid_lines: report.invalid_lines,
    }
}
