use std::error::Error;

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use midir::{MidiInputPorts, MidiInput, PortInfoError};

pub const PROGRAM_NAME: &'static str = "midimon";

pub fn get_best_matching_idx(inp: &MidiInput, v: &MidiInputPorts, pat: &str) -> Result<Option<usize>, PortInfoError> {
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

pub fn rewrap<T>(result: Result<T, impl Error>, verbosity: i32, code_if_err: i32, base_desc: &str) -> Result<T, (i32, String)> {
    match result {
        Ok(v) => Ok(v),
        Err(e) => {
            let mut err = String::new();
            if verbosity > -1 { err.push_str(base_desc) }
            if verbosity > 1 { err.push_str(&e.to_string()) }
            Err((code_if_err, err))
        }
    }
}

pub type MidiMessage = (u64, [u8; 3]);

