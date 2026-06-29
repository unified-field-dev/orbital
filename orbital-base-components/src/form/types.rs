#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputType {
    #[default]
    Text,
    Password,
    Search,
    Tel,
    Url,
    Email,
    Time,
    Date,
    DatetimeLocal,
    Month,
    Week,
    Number,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Email => "email",
            Self::Time => "time",
            Self::Date => "date",
            Self::DatetimeLocal => "datetime-local",
            Self::Month => "month",
            Self::Week => "week",
            Self::Number => "number",
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum InputSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl InputSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SelectSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SelectSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum FieldOrientation {
    Horizontal,
    #[default]
    Vertical,
}

impl FieldOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum LabelSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl LabelSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum LabelWeight {
    #[default]
    Regular,
    Semibold,
}

impl LabelWeight {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Regular => "regular",
            Self::Semibold => "semibold",
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum CheckboxSize {
    #[default]
    Medium,
    Large,
}

impl CheckboxSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

#[derive(Clone, Default)]
pub enum TextareaResize {
    #[default]
    None,
    Both,
    Horizontal,
    Vertical,
}

impl TextareaResize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Both => "both",
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TextareaSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl TextareaSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
