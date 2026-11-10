use std::collections::HashSet;

use crate::errors::PatternError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Segment {
    Literal(String),
    Slot { name: String, optional: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    segments: Vec<Segment>,
}

impl Pattern {
    pub fn parse(src: &str) -> Result<Self, PatternError> {
        let bytes = src.as_bytes();
        let mut i = 0;
        let mut segments = Vec::new();
        let mut seen = HashSet::new();

        while i < bytes.len() {
            if bytes[i].is_ascii_whitespace() {
                i += 1;
                continue;
            }
            let start = i;
            if src[i..].starts_with('{') {
                let close = src[i + 1..]
                    .find("}")
                    .ok_or(PatternError::Unclosed(start))?;
                let inner = src[i + 1..i + 1 + close].trim();
                let (name, optional) = match inner.strip_suffix('?') {
                    Some(n) => (n.trim_end(), true),
                    None => (inner, false),
                };
                if name.is_empty() {
                    return Err(PatternError::EmptySlot(start));
                }
                if !valid_name(name) {
                    return Err(PatternError::InvalidSlotName {
                        name: name.into(),
                        at: start,
                    });
                }
                if !seen.insert(name.to_string()) {
                    return Err(PatternError::DuplicateSlot(name.into()));
                }
                segments.push(Segment::Slot {
                    name: name.into(),
                    optional,
                });
                i += 1 + close + 1;
            } else {
                while i < bytes.len()
                    && !bytes[i].is_ascii_whitespace()
                    && !src[i..].starts_with("{")
                {
                    i += 1;
                }
                let lit = &src[start..i];
                if lit.starts_with('-')
                    || lit.contains('{')
                    || lit.contains('}')
                    || lit.ends_with('?')
                {
                    return Err(PatternError::InvalidLiteral(lit.into()));
                }
                segments.push(Segment::Literal(lit.into()));
            }
            // whatever we just read must be followed by whitespace or the end
            if i < bytes.len() && !bytes[i].is_ascii_whitespace() {
                return Err(PatternError::NotSeparated(i));
            }
        }

        if segments.is_empty() {
            return Err(PatternError::Empty);
        }
        Ok(Self { segments })
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    /// slot names in the order they appear on the command line
    pub fn slots(&self) -> impl Iterator<Item = &str> {
        self.segments.iter().filter_map(|s| match s {
            Segment::Slot { name, .. } => Some(name.as_str()),
            Segment::Literal(_) => None,
        })
    }
}

fn valid_name(s: &str) -> bool {
    let mut chars = s.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;
    use Segment::*;

    #[test]
    fn parses_slots_and_literals() {
        let slot = |name: &str, optional| Slot {
            name: name.into(),
            optional,
        };
        let p = Pattern::parse("api {module}   { resource? } {verb ?}").unwrap();
        assert_eq!(
            p.segments(),
            &[
                Literal("api".into()),
                slot("module", false),
                slot("resource", true),
                slot("verb", true)
            ]
        );
        assert_eq!(
            p.slots().collect::<Vec<_>>(),
            ["module", "resource", "verb"]
        );
    }

    #[test]
    fn rejects_bad_patterns() {
        assert_eq!(Pattern::parse("   "), Err(PatternError::Empty));
        assert_eq!(Pattern::parse("{ a } { b"), Err(PatternError::Unclosed(6)));
        assert_eq!(Pattern::parse("{ }"), Err(PatternError::EmptySlot(0)));
        assert_eq!(
            Pattern::parse("{ a } { a }"),
            Err(PatternError::DuplicateSlot("a".into()))
        );
        assert_eq!(Pattern::parse("x{ a }"), Err(PatternError::NotSeparated(1)));
        assert_eq!(Pattern::parse("{ a }x"), Err(PatternError::NotSeparated(5)));
        assert!(matches!(
            Pattern::parse("{ 1a }"),
            Err(PatternError::InvalidSlotName { .. })
        ));
        assert!(matches!(
            Pattern::parse("--x { a }"),
            Err(PatternError::InvalidLiteral(_))
        ));
        assert!(matches!(
            Pattern::parse("api? { a }"),
            Err(PatternError::InvalidLiteral(_))
        ));
        assert_eq!(Pattern::parse("{ ? }"), Err(PatternError::EmptySlot(0)));
    }
}
