use wasm_bindgen::prelude::*;
use robotstxt::DefaultMatcher;

#[wasm_bindgen]
pub fn is_allowed(txt: &str, user_agent: &str, url: &str) -> bool {
    let mut matcher = DefaultMatcher::default();
    matcher.one_agent_allowed_by_robots(txt, user_agent, url)
}
