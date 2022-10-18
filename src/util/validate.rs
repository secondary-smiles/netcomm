use crate::Args;
use info_utils::prelude::*;
use std::net::IpAddr;

use crate::LogUtil;

impl Args {
    pub fn validate(&mut self) -> Result<(), String> {
        /* Checks:
        - PORT
        - If not --listen; PORT && DOMAIN
        - If --listen; PORT !DOMAIN
        - If DOMAIN; DOMAIN is valid URL or IP
        - Cannot set -v && -q
        */
        if self.port == None {
            return Err("<PORT> must be specified".to_string());
        }

        if !self.listen && self.domain == None {
            return Err("<DOMAIN> or --listen required".to_string());
        }
        if self.listen && self.domain != None {
            return Err(
                "<DOMAIN> cannot be specified with listener server".to_string(),
            );
        }

        if self.quiet && self.verbose {
            return Err(
                "cannot set \"verbose\" and \"quiet\" options at the same time".to_string(),
            );
        }
        if self.domain != None {
            self.log_v("Validating DOMAIN as url/ip..");
            self.domain = match make_valid_domain(self.domain.clone().eval()) {
                Ok(d) => Some(d),
                Err(e) => {
                    return Err(format!(
                        "{:?} is not a valid domain or IP for reason: \"{}\"",
                        self.domain.clone().eval(),
                        e
                    ));
                }
            }
        }

        Ok(())
    }
}

fn make_valid_domain(domain: String) -> Result<String, String> {
    /* Checks:
    IF DOMAIN
    - DOMAIN is alphanumeric
    - DOMAIN length is 1-63
    - DOMAIN does not start nor end with hyphen
    - Last TLD length is 2-6
    - Split by '.' at least 2
    */

    // Preprocess
    // Remove protocol
    let test_domain: String = domain.split("://").last().eval().to_string();
    // Remove appended paths
    let test_domain: String = test_domain.split('/').next().eval().to_string();

    // Check if valid IP
    if test_domain.parse::<IpAddr>().is_ok() {
        return Ok(test_domain);
    }

    // DOMAIN alphanumeric
    for c in test_domain.replace('.', "").replace('-', "").chars() {
        if !c.is_alphanumeric() {
            return Err(format!("{:?} is not a valid domain character", c));
        }
    }

    // DOMAIN length
    if test_domain.is_empty() || test_domain.len() > 63 {
        return Err(format!(
            "{:?} is not the correct length of a valid domain",
            test_domain
        ));
    }

    // No leading || trailing hyphen
    if test_domain.starts_with('-') || test_domain.ends_with('-') {
        return Err("Domains cannot start nor end with \"-\"".to_string());
    }

    // Split '.' length >= 2
    if test_domain.split('.').count() <= 1 {
        return Err(format!(
            "{:?} has an invalid number of segments when split by \".\"",
            test_domain
        ));
    }

    // Last TLD length >= 2
    if test_domain.split('.').last().eval().len() < 2
        || test_domain.split('.').last().eval().len() > 6
    {
        return Err(format!(
            "Top level domain {:?} is not the correct length for a valid TLD",
            test_domain.split('.').last().eval()
        ));
    }

    Ok(test_domain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_valid_domain_url() {
        // TODO: Split into multiple tests
        // Valid urls
        // Basic validation
        let test_url: String = "example.com".to_string();
        assert_eq!(make_valid_domain(test_url).eval(), "example.com");

        // Protocol removal
        let test_url: String = "https://example.com".to_string();
        assert_eq!(make_valid_domain(test_url).eval(), "example.com");

        // Postfixed path
        let test_url: String = "example.com/sub/path".to_string();
        assert_eq!(make_valid_domain(test_url).eval(), "example.com");

        // Invalid urls
        // Invalid characters
        let test_url: String = "example.'com".to_string(); // ' is an invalid char for a url
        assert_eq!(
            make_valid_domain(test_url).eval_or("invalid url".to_string()),
            "invalid url"
        );

        // Too long
        let test_url: String =
            "this-example-domain-is-way-way-way-too-long-to-be-a-valid-url-in-real-life.com"
                .to_string();
        assert_eq!(
            make_valid_domain(test_url).eval_or("invalid url".to_string()),
            "invalid url"
        );

        // post || prefixed url
        let test_url: String = "example.this-".to_string();
        assert_eq!(
            make_valid_domain(test_url).eval_or("invalid url".to_string()),
            "invalid url"
        );

        // Invalid TLD length
        let test_url: String = "example.thisistoolong".to_string();
        assert_eq!(
            make_valid_domain(test_url).eval_or("invalid url".to_string()),
            "invalid url"
        );

        // Invalid number of segments
        let test_url: String = "onlyonesegment".to_string();
        assert_eq!(
            make_valid_domain(test_url).eval_or("invalid url".to_string()),
            "invalid url"
        );
    }

    #[test]
    fn test_valid_domain_ip() {
        // Valid IPs
        let ipv4: String = "127.0.0.1".to_string();
        let ipv6: String = "2001:db8:3333:4444:5555:6666:7777:8888".to_string();
        assert_eq!(make_valid_domain(ipv4).eval(), "127.0.0.1");
        assert_eq!(
            make_valid_domain(ipv6).eval(),
            "2001:db8:3333:4444:5555:6666:7777:8888"
        );

        // Invalid IPs
        let ipv4: String = "127.0.0.1.0".to_string();
        let ipv6: String = "2001:db8:3333:4444:5555:6666:7777:8888:0".to_string();
        assert_eq!(
            make_valid_domain(ipv4).eval_or("invalid ip".to_string()),
            "invalid ip"
        );
        assert_eq!(
            make_valid_domain(ipv6).eval_or("invalid ip".to_string()),
            "invalid ip"
        );
    }
}