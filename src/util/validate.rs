use crate::Args;
use info_utils::prelude::*;
use std::net::IpAddr;

impl Args {
    pub fn validate(&mut self) -> Result<(), String> {
        /* Checks:
        - PORT
        - If not --listen; PORT && DOMAIN
        - If --listen; PORT !DOMAIN
        - If DOMAIN; DOMAIN is valid URL or IP
        - Cannot set -v && -q
        */
        self.domain = make_valid_domain(self.domain.clone()).eval_or_else(|e| {
            terror!("Parse error: {e}");
        });

        Ok(())
    }
}

fn make_valid_domain(domain: String) -> Result<String, String> {
    // Preprocess
    // Remove protocol
    let test_domain: String = domain.split("://").last().eval().to_string();
    // Remove appended paths
    let test_domain: String = test_domain.split('/').next().eval().to_string();

    Ok(test_domain)
}