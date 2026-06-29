/// Avatar background color preset aligned with Orbital palette tokens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarColor {
    #[default]
    Neutral,
    /// Stable hash from name or `id_for_color` into the named palette.
    Colorful,
    Brand,
    Crimson,
    Azure,
    Forest,
    Tangerine,
    Plum,
    Ruby,
    Marigold,
    Ash,
}

impl AvatarColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Colorful => "colorful",
            Self::Brand => "brand",
            Self::Crimson => "crimson",
            Self::Azure => "azure",
            Self::Forest => "forest",
            Self::Tangerine => "tangerine",
            Self::Plum => "plum",
            Self::Ruby => "ruby",
            Self::Marigold => "marigold",
            Self::Ash => "ash",
        }
    }

    /// Named palette entries used for `Colorful` mode hashing.
    pub const COLORFUL_PALETTE: &'static [AvatarColor] = &[
        Self::Crimson,
        Self::Azure,
        Self::Forest,
        Self::Tangerine,
        Self::Plum,
        Self::Ruby,
        Self::Marigold,
        Self::Ash,
    ];

    /// Resolve a concrete named color for rendering.
    pub fn resolve(
        self,
        name: Option<&str>,
        initials: Option<&str>,
        id_for_color: Option<&str>,
    ) -> Self {
        if self != Self::Colorful {
            return self;
        }
        let key = name
            .filter(|s| !s.is_empty())
            .or(initials.filter(|s| !s.is_empty()))
            .or(id_for_color.filter(|s| !s.is_empty()))
            .unwrap_or("");
        let idx = color_hash(key) % Self::COLORFUL_PALETTE.len();
        Self::COLORFUL_PALETTE[idx]
    }
}

/// Stable hash for colorful avatar assignment (djb2 over lowercase bytes).
pub fn color_hash(key: &str) -> usize {
    let mut hash: u64 = 5381;
    for byte in key.to_lowercase().bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(u64::from(byte));
    }
    hash as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_hash_is_deterministic() {
        assert_eq!(color_hash("Jane Doe"), color_hash("Jane Doe"));
        assert_eq!(color_hash("Jane Doe"), color_hash("jane doe"));
    }

    #[test]
    fn color_hash_differs_for_different_names() {
        assert_ne!(color_hash("Alice"), color_hash("Bob"));
    }

    #[test]
    fn colorful_resolves_to_named_palette() {
        let resolved = AvatarColor::Colorful.resolve(Some("Jane Doe"), None, None);
        assert!(AvatarColor::COLORFUL_PALETTE.contains(&resolved));
    }
}
