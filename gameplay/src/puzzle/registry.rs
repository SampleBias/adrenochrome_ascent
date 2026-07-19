//! Global puzzle flag registry + compound condition evaluator (TODO-008).

use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Named boolean flags set by puzzle solves / interactables.
#[derive(Resource, Debug, Clone, Default, Serialize, Deserialize)]
pub struct PuzzleRegistry {
    pub flags: HashMap<String, bool>,
}

impl PuzzleRegistry {
    pub fn get(&self, name: &str) -> bool {
        self.flags.get(name).copied().unwrap_or(false)
    }

    pub fn set(&mut self, name: impl Into<String>, value: bool) {
        self.flags.insert(name.into(), value);
    }

    pub fn clear(&mut self) {
        self.flags.clear();
    }

    /// Evaluate a simple boolean expression over flag names.
    ///
    /// Supports `&&`, `||`, `!`, parentheses, and identifiers.
    /// Empty / missing expression → `true`.
    pub fn evaluate(&self, expr: &str) -> bool {
        let expr = expr.trim();
        if expr.is_empty() {
            return true;
        }
        match parse_or(expr, self) {
            Ok((val, rest)) if rest.trim().is_empty() => val,
            Ok((_, rest)) => {
                warn!("Trailing junk in puzzle condition '{expr}': '{rest}'");
                false
            }
            Err(e) => {
                warn!("Bad puzzle condition '{expr}': {e}");
                false
            }
        }
    }
}

type ParseResult<'a> = Result<(bool, &'a str), String>;

fn parse_or<'a>(input: &'a str, reg: &PuzzleRegistry) -> ParseResult<'a> {
    let (mut left, mut rest) = parse_and(input, reg)?;
    loop {
        let t = rest.trim_start();
        if let Some(next) = t.strip_prefix("||") {
            let (right, after) = parse_and(next, reg)?;
            left = left || right;
            rest = after;
        } else {
            return Ok((left, rest));
        }
    }
}

fn parse_and<'a>(input: &'a str, reg: &PuzzleRegistry) -> ParseResult<'a> {
    let (mut left, mut rest) = parse_unary(input, reg)?;
    loop {
        let t = rest.trim_start();
        if let Some(next) = t.strip_prefix("&&") {
            let (right, after) = parse_unary(next, reg)?;
            left = left && right;
            rest = after;
        } else {
            return Ok((left, rest));
        }
    }
}

fn parse_unary<'a>(input: &'a str, reg: &PuzzleRegistry) -> ParseResult<'a> {
    let t = input.trim_start();
    if let Some(next) = t.strip_prefix('!') {
        let (val, rest) = parse_primary(next, reg)?;
        return Ok((!val, rest));
    }
    parse_primary(t, reg)
}

fn parse_primary<'a>(input: &'a str, reg: &PuzzleRegistry) -> ParseResult<'a> {
    let t = input.trim_start();
    if let Some(inner) = t.strip_prefix('(') {
        let (val, rest) = parse_or(inner, reg)?;
        let rest = rest.trim_start();
        let rest = rest
            .strip_prefix(')')
            .ok_or_else(|| "missing ')'".to_string())?;
        return Ok((val, rest));
    }
    let (ident, rest) = take_ident(t)?;
    Ok((reg.get(ident), rest))
}

fn take_ident(input: &str) -> Result<(&str, &str), String> {
    let bytes = input.as_bytes();
    if bytes.is_empty() || !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_') {
        return Err(format!("expected identifier, got '{input}'"));
    }
    let mut i = 1;
    while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
        i += 1;
    }
    Ok((&input[..i], &input[i..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_compound_flags() {
        let mut reg = PuzzleRegistry::default();
        reg.set("has_keycard", true);
        reg.set("power_restored", false);
        assert!(reg.evaluate("has_keycard"));
        assert!(!reg.evaluate("has_keycard && power_restored"));
        reg.set("power_restored", true);
        assert!(reg.evaluate("has_keycard && power_restored"));
        assert!(reg.evaluate("!missing_flag || has_keycard"));
        assert!(reg.evaluate("(has_keycard && power_restored)"));
    }
}
