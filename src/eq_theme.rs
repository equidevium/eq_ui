use dioxus::hooks::{use_context, use_context_provider};
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum EqTheme {
    #[default]
    Unghosty,
    Burgundy,
    Gold,
    PurplePink,
    Monochrome,
    Watermelon,
    Sunset,
    Ocean,
    Spacetime,
    Gruvbox,
    Monokai,
    Hellas,
    Egypt,
    Dometrain,
    Catppuccin,
    Dracula,
    Nord,
    OneDark,
    RosePine,
    SolarizedDark,
    TokyoNight,
    /// User-provided CSS injected at runtime.
    Custom(String),
}

impl EqTheme {
    pub fn css_content(&self) -> Option<&'static str> {
        match self {
            EqTheme::Unghosty   => Some(include_str!("../assets/theme/unghosty.css")),
            EqTheme::Burgundy   => Some(include_str!("../assets/theme/burgundy.css")),
            EqTheme::Gold       => Some(include_str!("../assets/theme/gold.css")),
            EqTheme::PurplePink => Some(include_str!("../assets/theme/purple_pink.css")),
            EqTheme::Monochrome => Some(include_str!("../assets/theme/monochrome.css")),
            EqTheme::Watermelon => Some(include_str!("../assets/theme/watermelon.css")),
            EqTheme::Sunset     => Some(include_str!("../assets/theme/sunset.css")),
            EqTheme::Ocean      => Some(include_str!("../assets/theme/ocean.css")),
            EqTheme::Spacetime  => Some(include_str!("../assets/theme/spacetime.css")),
            EqTheme::Gruvbox    => Some(include_str!("../assets/theme/gruvbox.css")),
            EqTheme::Monokai    => Some(include_str!("../assets/theme/monokai.css")),
            EqTheme::Hellas     => Some(include_str!("../assets/theme/hellas.css")),
            EqTheme::Egypt      => Some(include_str!("../assets/theme/egypt.css")),
            EqTheme::Dometrain  => Some(include_str!("../assets/theme/dometrain.css")),
            EqTheme::Catppuccin => Some(include_str!("../assets/theme/catppuccin.css")),
            EqTheme::Dracula    => Some(include_str!("../assets/theme/dracula.css")),
            EqTheme::Nord       => Some(include_str!("../assets/theme/nord.css")),
            EqTheme::OneDark    => Some(include_str!("../assets/theme/one_dark.css")),
            EqTheme::RosePine   => Some(include_str!("../assets/theme/rose_pine.css")),
            EqTheme::SolarizedDark => Some(include_str!("../assets/theme/solarized_dark.css")),
            EqTheme::TokyoNight => Some(include_str!("../assets/theme/tokyo_night.css")),
            EqTheme::Custom(_)  => None,
        }
    }

    pub fn custom_css(&self) -> Option<&str> {
        match self {
            EqTheme::Custom(css) => Some(css.as_str()),
            _ => None,
        }
    }

    pub fn build_in_variants() -> Vec<(&'static str, EqTheme)> {
        vec![
            ("Unghosty", EqTheme::Unghosty),
            ("Burgundy", EqTheme::Burgundy),
            ("Gold", EqTheme::Gold),
            ("PurplePink", EqTheme::PurplePink),
            ("Monochrome", EqTheme::Monochrome),
            ("Watermelon", EqTheme::Watermelon),
            ("Sunset", EqTheme::Sunset),
            ("Ocean", EqTheme::Ocean),
            ("Spacetime", EqTheme::Spacetime),
            ("Gruvbox", EqTheme::Gruvbox),
            ("Monokai", EqTheme::Monokai),
            ("Hellas", EqTheme::Hellas),
            ("Egypt", EqTheme::Egypt),
            ("Dometrain", EqTheme::Dometrain),
            ("Catppuccin", EqTheme::Catppuccin),
            ("Dracula", EqTheme::Dracula),
            ("Nord", EqTheme::Nord),
            ("OneDark", EqTheme::OneDark),
            ("RosePine", EqTheme::RosePine),
            ("SolarizedDark", EqTheme::SolarizedDark),
            ("TokyoNight", EqTheme::TokyoNight),
        ]
    }

    // Helper method to create a custom theme from raw CSS string
    pub fn use_theme_provider() -> Signal<EqTheme> {
        use_context_provider(|| Signal::new(EqTheme::default()))
    }

    pub fn use_theme() -> Signal<EqTheme> {
        use_context::<Signal<EqTheme>>()
    }

    pub fn set_theme(theme: EqTheme) {
        let mut current = Self::use_theme();
        current.set(theme);
    }

    pub fn set_custom_theme(css: String) {
        let mut current = Self::use_theme();
        current.set(EqTheme::Custom(css));
    }
}
