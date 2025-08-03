use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

pub struct Tools;

impl Tools {
    pub fn fuzzy_search<'a>(licenses: &'a [String], query: &str) -> Vec<(&'a str, i64)> {
        let matcher = SkimMatcherV2::default();
        let mut matches = Vec::new();

        for license in licenses {
            if let Some(score) = matcher.fuzzy_match(license, query) {
                matches.push((license.as_str(), score));
            }
        }

        // Sort by score (higher is better)
        matches.sort_by(|a, b| b.1.cmp(&a.1));

        // Return only top 3 matches
        matches.into_iter().take(3).collect()
    }
}
