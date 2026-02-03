#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AvatarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl AvatarSize {
    pub fn class_name(self) -> &'static str {
        match self {
            AvatarSize::Sm => "ui-avatar--sm",
            AvatarSize::Md => "ui-avatar--md",
            AvatarSize::Lg => "ui-avatar--lg",
        }
    }
}

pub(super) fn initials_from_name(name: &str) -> Option<String> {
    let name = name.trim();
    if name.is_empty() {
        return None;
    }

    let mut words = name.split_whitespace().filter(|w| !w.is_empty());
    let first = words.next()?;
    let last = words.next_back().unwrap_or(first);

    let first_char = first.chars().next()?;
    let last_char = (last != first).then(|| last.chars().next()).flatten();

    let mut initials = String::new();
    initials.push(first_char);
    if let Some(last_char) = last_char {
        initials.push(last_char);
    }

    Some(initials.to_uppercase())
}
