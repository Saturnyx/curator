use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub struct Tools;

impl Tools {
    pub fn fuzzy_search<'a>(licenses: &'a [String], query: &str) -> Vec<(&'a str, i64)> {
        let matcher = SkimMatcherV2::default();
        let mut matches = Vec::with_capacity(std::cmp::min(licenses.len(), 10)); // Pre-allocate

        for license in licenses {
            if let Some(score) = matcher.fuzzy_match(license, query) {
                matches.push((license.as_str(), score));
            }
        }

        if matches.len() > 3 {
            matches.select_nth_unstable_by(2, |a, b| b.1.cmp(&a.1));
            matches.truncate(3);
            matches.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        } else {
            matches.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        }

        matches
    }
}
