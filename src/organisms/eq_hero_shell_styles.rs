//! Style constants for EqHeroShell component.

pub const HERO_SHELL: &str =
    "py-12 md:py-28 bg-[var(--gradient-background)]";

pub const HERO_TITLE: &str =
    "text-3xl md:text-5xl font-semibold tracking-tight text-[var(--color-label-primary)]";

pub const HERO_SUBTITLE: &str =
    "mt-3 md:mt-4 max-w-2xl text-base md:text-lg text-[var(--color-label-secondary)]";

pub const HERO_ACTIONS: &str =
    "mt-6 md:mt-8 flex flex-wrap gap-3 md:gap-4";

pub const HERO_SHELL_RELATIVE: &str =
    "relative overflow-hidden";
pub const HERO_BG: &str =
    "absolute inset-0 w-full h-full";

pub const HERO_OVERLAY: &str =
    "absolute inset-0 bg-black/50";
pub const HERO_CONTENT: &str =
    "relative z-10";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("HERO_SHELL", HERO_SHELL),
        ("HERO_TITLE", HERO_TITLE),
        ("HERO_SUBTITLE", HERO_SUBTITLE),
        ("HERO_ACTIONS", HERO_ACTIONS),
        ("HERO_SHELL_RELATIVE", HERO_SHELL_RELATIVE),
        ("HERO_BG", HERO_BG),
        ("HERO_OVERLAY", HERO_OVERLAY),
        ("HERO_CONTENT", HERO_CONTENT),
    ]
}