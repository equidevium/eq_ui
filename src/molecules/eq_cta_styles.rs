//! Style constants for EqCta.

pub const CTA: &str =
    "rounded-xl border border-[var(--color-card-border)] \
     bg-[var(--color-tertiary-dark)]/60 \
     p-4 md:p-6 flex flex-col gap-4 md:flex-row md:items-center md:justify-between";

pub const CTA_CENTERED: &str =
    "rounded-xl border border-[var(--color-card-border)] \
     bg-[var(--color-tertiary-dark)]/60 \
     p-4 md:p-6 flex flex-col gap-4 items-center text-center";

pub const TEXT_GROUP: &str = "space-y-1";

pub const TITLE: &str =
    "text-lg font-semibold text-[var(--color-label-primary)]";

pub const DESCRIPTION: &str =
    "text-sm text-[var(--color-label-secondary)]";

pub const ACTION: &str = "shrink-0";

pub const ACTION_CENTERED: &str = "shrink-0 mt-2";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("CTA", CTA),
        ("CTA_CENTERED", CTA_CENTERED),
        ("TEXT_GROUP", TEXT_GROUP),
        ("TITLE", TITLE),
        ("DESCRIPTION", DESCRIPTION),
        ("ACTION", ACTION),
        ("ACTION_CENTERED", ACTION_CENTERED),
    ]
}
