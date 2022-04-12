use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use midir::{MidiInputPorts, MidiInput};

pub fn get_best_matching_idx(inp: &MidiInput, v: &MidiInputPorts, pat: &str) -> Result<Option<usize>, Box<dyn std::error::Error>> {
    let matcher = SkimMatcherV2::default();
    let mut scores: Vec<(usize, i64)> = Vec::new();

    for (i, s) in v.iter().enumerate() {
        let name = inp.port_name(s)?;
        match matcher.fuzzy_match(&name, &pat) {
            Some(score) => scores.push((i, score)),
            None => {}
        }
    }

    scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    if scores.len() == 0 {
        Ok(None)
    }
    else {
        let (idx, _) = scores.pop().unwrap();
        Ok(Some(idx))
    }
}