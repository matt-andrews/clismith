#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bump {
    None,
    Patch,
    Minor,
    Major,
}

#[derive(Debug)]
pub struct Change {
    pub path: String,
    pub bump: Bump,
    pub detail: String,
}

pub trait Diff {
    /// Compare `self` against `new`, pushing any changes into `out`.
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<Change>);
}

pub fn cmp_field_with<T: PartialEq + std::fmt::Debug>(
    out: &mut Vec<Change>,
    path: &str,
    name: &str,
    old: &T,
    new: &T,
    bump: impl Fn(&T, &T) -> Bump,
) {
    if old != new {
        out.push(Change {
            path: format!("{path}.{name}"),
            bump: bump(old, new),
            detail: format!("{old:?} -> {new:?}"),
        });
    }
}

pub fn cmp_field<T: PartialEq + std::fmt::Debug>(
    out: &mut Vec<Change>,
    path: &str,
    name: &str,
    old: &T,
    new: &T,
    bump: Bump,
) {
    cmp_field_with(out, path, name, old, new, |_, _| bump);
}

pub fn diff_by_key<'a, T: Diff, K: PartialEq + std::fmt::Debug>(
    old: &'a [T],
    new: &'a [T],
    path: &str,
    out: &mut Vec<Change>,
    key: impl Fn(&'a T) -> K,
) {
    for o in old {
        match new.iter().find(|n| key(n) == key(o)) {
            Some(n) => o.diff(n, &format!("{path}[{:?}]", key(o)), out),
            None => out.push(Change {
                path: format!("{path}[{:?}]", key(o)),
                bump: Bump::Major,
                detail: "removed".into(),
            }),
        }
    }
    for n in new.iter().filter(|n| !old.iter().any(|o| key(o) == key(n))) {
        out.push(Change {
            path: format!("{path}[{:?}]", key(n)),
            bump: Bump::Minor,
            detail: "added".into(),
        });
    }
}

pub fn diff_opt_by_key<'a, T: Diff, K: PartialEq + std::fmt::Debug>(
    old: &'a Option<Vec<T>>,
    new: &'a Option<Vec<T>>,
    path: &str,
    out: &mut Vec<Change>,
    key: impl Fn(&'a T) -> K,
) {
    diff_by_key(
        old.as_deref().unwrap_or_default(),
        new.as_deref().unwrap_or_default(),
        path,
        out,
        key,
    );
}

pub fn diff_string_list(
    old: &Option<Vec<String>>,
    new: &Option<Vec<String>>,
    path: &str,
    out: &mut Vec<Change>,
    added: Bump,
    removed: Bump,
) {
    let old = old.as_deref().unwrap_or_default();
    let new = new.as_deref().unwrap_or_default();

    for s in old.iter().filter(|s| !new.contains(s)) {
        out.push(Change {
            path: format!("{path}[{s:?}]"),
            bump: removed,
            detail: "removed".into(),
        });
    }
    for s in new.iter().filter(|s| !old.contains(s)) {
        out.push(Change {
            path: format!("{path}[{s:?}]"),
            bump: added,
            detail: "added".into(),
        });
    }
}
