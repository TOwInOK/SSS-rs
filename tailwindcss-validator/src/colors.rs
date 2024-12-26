/// Base Tailwind CSS colors
pub static BASE_COLORS: [&str; 23] = [
    "slate",
    "gray",
    "zinc",
    "neutral",
    "stone",
    "red",
    "orange",
    "amber",
    "yellow",
    "lime",
    "green",
    "emerald",
    "teal",
    "cyan",
    "sky",
    "blue",
    "indigo",
    "violet",
    "purple",
    "fuchsia",
    "pink",
    "rose",
    "transparent",
];

/// Valid color intensities for Tailwind CSS
pub static VALID_INTENSITIES: [&str; 11] = [
    "50", "100", "200", "300", "400", "500", "600", "700", "800", "900", "950",
];

/// Special colors that don't require intensity
pub static SPECIAL_COLORS: [&str; 3] = ["white", "black", "transparent"];
