use super::define_css_token_enum;

define_css_token_enum! {
    /// Motion duration tokens from the active theme (`--orb-motion-duration-*`).
    pub enum MotionDuration {
        UltraFast => "--orb-motion-duration-2xs",
        Faster => "--orb-motion-duration-xs",
        Normal => "--orb-motion-duration-md",
        Gentle => "--orb-motion-duration-lg",
        Slow => "--orb-motion-duration-xl",
    }
}

define_css_token_enum! {
    /// Motion easing curve tokens from the active theme (`--orb-motion-ease-*`).
    pub enum MotionCurve {
        AccelerateMid => "--orb-motion-ease-accelerate",
        DecelerateMax => "--orb-motion-ease-decelerate-strong",
        DecelerateMid => "--orb-motion-ease-decelerate",
        EasyEase => "--orb-motion-ease-standard",
        EasyEaseMax => "--orb-motion-ease-emphasis",
    }
}
