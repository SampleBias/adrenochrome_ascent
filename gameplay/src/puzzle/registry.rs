//! Global puzzle flag / counter registry + condition DSL (TODO-008 / TODO-026).

use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Named boolean flags + integer counters for biometric / DNA puzzles.
#[derive(Resource, Debug, Clone, Default, Serialize, Deserialize)]
pub struct PuzzleRegistry {
    pub flags: HashMap<String, bool>,
    pub counters: HashMap<String, i32>,
}

impl PuzzleRegistry {
    pub fn get(&self, name: &str) -> bool {
        self.flags.get(name).copied().unwrap_or(false)
    }

    pub fn set(&mut self, name: impl Into<String>, value: bool) {
        self.flags.insert(name.into(), value);
    }

    pub fn counter(&self, name: &str) -> i32 {
        self.counters.get(name).copied().unwrap_or(0)
    }

    pub fn add_counter(&mut self, name: impl Into<String>, amount: i32) {
        let name = name.into();
        let v = self.counter(&name) + amount;
        self.counters.insert(name, v);
    }

    pub fn set_counter(&mut self, name: impl Into<String>, value: i32) {
        self.counters.insert(name.into(), value);
    }

    pub fn clear(&mut self) {
        self.flags.clear();
        self.counters.clear();
    }

    /// Evaluate a boolean expression over flags and counter comparisons.
    ///
    /// Supports `&&`, `||`, `!`, parentheses, identifiers (flags),
    /// and comparisons: `collected_limb == 3`, `collected_limb >= 2`.
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
    let rest = rest.trim_start();
    if rest.starts_with("==")
        || rest.starts_with(">=")
        || rest.starts_with("<=")
        || rest.starts_with('>')
        || rest.starts_with('<')
    {
        return parse_comparison(ident, rest, reg);
    }
    // Bare identifier: flag true, or counter > 0 as truthy.
    Ok((reg.get(ident) || reg.counter(ident) > 0, rest))
}

fn parse_comparison<'a>(
    ident: &str,
    rest: &'a str,
    reg: &PuzzleRegistry,
) -> ParseResult<'a> {
    let left = reg.counter(ident);
    if let Some(after) = rest.strip_prefix("==") {
        let (n, after) = take_i32(after.trim_start())?;
        return Ok((left == n, after));
    }
    if let Some(after) = rest.strip_prefix(">=") {
        let (n, after) = take_i32(after.trim_start())?;
        return Ok((left >= n, after));
    }
    if let Some(after) = rest.strip_prefix("<=") {
        let (n, after) = take_i32(after.trim_start())?;
        return Ok((left <= n, after));
    }
    if let Some(after) = rest.strip_prefix('>') {
        let (n, after) = take_i32(after.trim_start())?;
        return Ok((left > n, after));
    }
    if let Some(after) = rest.strip_prefix('<') {
        let (n, after) = take_i32(after.trim_start())?;
        return Ok((left < n, after));
    }
    Err(format!("expected comparison after '{ident}'"))
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

fn take_i32(input: &str) -> Result<(i32, &str), String> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Err("expected number".into());
    }
    let mut i = 0;
    if bytes[0] == b'-' {
        i = 1;
    }
    let start = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == start {
        return Err(format!("expected number, got '{input}'"));
    }
    let n: i32 = input[..i]
        .parse::<i32>()
        .map_err(|e: std::num::ParseIntError| e.to_string())?;
    Ok((n, &input[i..]))
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

    #[test]
    fn evaluates_counter_comparisons() {
        let mut reg = PuzzleRegistry::default();
        reg.set_counter("collected_limb", 2);
        assert!(!reg.evaluate("collected_limb == 3"));
        assert!(reg.evaluate("collected_limb >= 2"));
        reg.add_counter("collected_limb", 1);
        assert!(reg.evaluate("collected_limb == 3"));
        assert!(reg.evaluate("collected_limb >= 3 && has_keycard || collected_limb == 3"));
    }
}
