//! Style constants for EqHeroShell component.

pub const HERO_SHELL: &str =
    "py-20 md:py-28 bg-[var(--gradient-background)]";

pub const HERO_TITLE: &str =
    "text-4xl md:text-5xl font-semibold tracking-tight text-[var(--color-label-primary)]";

pub const HERO_SUBTITLE: &str =
    "mt-4 max-w-2xl text-lg text-[var(--color-label-secondary)]";

pub const HERO_ACTIONS: &str =
    "mt-8 flex gap-4";

pub const HERO_SHELL_RELATIVE: &str =
    "relative overflow-hidden";
pub const HERO_BG: &str =
    "absolute inset-0 w-full h-full";

//TODO: we will need to review more customizability on this part.
pub const HERO_OVERLAY: &str =
    "absolute inset-0 bg-black/50";  // darkens image so text is readable
pub const HERO_CONTENT: &str =
    "relative z-10";