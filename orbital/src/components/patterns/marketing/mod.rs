pub mod feature_section;
pub mod hero_section;

// Re-export marketing patterns
pub use feature_section::{
    FeatureSection, FeatureSectionPreview, FeatureVariant, FEATURESECTION_DOC,
    FEATURESECTION_PREVIEW_REGISTRATION, FEATURESECTION_PROPS,
};
pub use hero_section::{
    HeightUnit, HeroCta, HeroSection, HeroSectionPreview, HEROSECTION_DOC,
    HEROSECTION_PREVIEW_REGISTRATION, HEROSECTION_PROPS,
};
