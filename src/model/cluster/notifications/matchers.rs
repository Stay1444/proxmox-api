use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PveMatcher {
    pub name: String, //Name of the matcher.
    pub origin: Option<String>, //Show if this entry was created by a user or was built-in.
    pub comment: Option<String>, //Comment.
    pub disable: Option<bool>, //Disable this matcher.
    #[serde(rename = "invert-match")]
    pub invert_match: Option<bool>, //Invert match of the whole matcher.
    #[serde(rename = "match-calendar")]
    pub match_calendar: Option<Vec<String>>, //Match notification timestamp.
    #[serde(rename = "match-field")]
    pub match_field: Option<Vec<String>>, //Metadata fields to match (regex or exact match). Must be in the form (regex|exact):<field>=<value>.
    #[serde(rename = "match-security")]
    pub match_security: Option<Vec<String>>, //Notification severities to match.
    pub mode: Option<MatcherMode>, //Choose between 'all' and 'any' for when multiple properties are specified.
    pub target: Option<Vec<String>> //Targets to notify on match.
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MatcherMode {
    All,
    Any
}