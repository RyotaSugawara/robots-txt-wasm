use wasm_bindgen::prelude::*;
use robotstxt::{DefaultMatcher,RobotsParseHandler,parse_robotstxt};

#[derive(Default)]
struct RobotsStatsReporter {
    valid_lines: Vec<u32>,
}

impl RobotsStatsReporter {
    fn digest(&mut self, line_num: u32) {
        self.valid_lines.push(line_num);
    }
}

impl RobotsParseHandler for RobotsStatsReporter {
    fn handle_robots_start(&mut self) {
        let n = 2083 * 8; // max line
        self.valid_lines = Vec::with_capacity(n);
    }

    fn handle_robots_end(&mut self) {}

    fn handle_user_agent(&mut self, line_num: u32, _user_agent: &str) {
        self.digest(line_num);
    }

    fn handle_allow(&mut self, line_num: u32, _value: &str) {
        self.digest(line_num);
    }

    fn handle_disallow(&mut self, line_num: u32, _value: &str) {
        self.digest(line_num);
    }

    fn handle_sitemap(&mut self, line_num: u32, _value: &str) {
        self.digest(line_num);
    }

    fn handle_unknown_action(&mut self, _line_num: u32, _action: &str, _value: &str) {}
}

#[wasm_bindgen]
pub fn is_allowed(txt: &str, user_agent: &str, url: &str) -> bool {
    let mut matcher = DefaultMatcher::default();
    matcher.one_agent_allowed_by_robots(txt, user_agent, url)
}

#[wasm_bindgen]
pub fn validate(txt: &str) -> Vec<u32> {
    let mut report = RobotsStatsReporter::default();
    parse_robotstxt(txt, &mut report);
    report.valid_lines
}
