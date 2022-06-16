use druid::{theme::*, FontDescriptor, FontFamily};
use druid::{Color, Env, Key};

pub const NEUTRAL_0: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_100");
pub const NEUTRAL_100: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_100");
pub const NEUTRAL_200: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_200");
pub const NEUTRAL_300: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_300");
pub const NEUTRAL_400: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_400");
pub const NEUTRAL_500: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_500");
pub const NEUTRAL_600: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_600");
pub const NEUTRAL_700: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_700");
pub const NEUTRAL_800: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_700");
pub const NEUTRAL_900: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_700");
pub const NEUTRAL_1000: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_700");

pub const THEME_100: Key<Color> = Key::new("online.smethwick.clive.theme.theme_100");
pub const THEME_200: Key<Color> = Key::new("online.smethwick.clive.theme.theme_200");
pub const THEME_300: Key<Color> = Key::new("online.smethwick.clive.theme.theme_300");
pub const THEME_400: Key<Color> = Key::new("online.smethwick.clive.theme.theme_400");
pub const THEME_500: Key<Color> = Key::new("online.smethwick.clive.theme.theme_500");
pub const THEME_600: Key<Color> = Key::new("online.smethwick.clive.theme.theme_600");
pub const THEME_700: Key<Color> = Key::new("online.smethwick.clive.theme.theme_700");
pub const THEME_800: Key<Color> = Key::new("online.smethwick.clive.theme.theme_700");
pub const THEME_900: Key<Color> = Key::new("online.smethwick.clive.theme.theme_700");

pub const FONT: Key<FontDescriptor> = Key::new("online.smethwick.clive.theme.ui_font");

pub fn setup(env: &mut Env) {
    // colour definitons
    env.set(NEUTRAL_0, Color::from_rgba32_u32(0x000000ff));
    env.set(NEUTRAL_100, Color::from_rgba32_u32(0x0e0e0eff));
    env.set(NEUTRAL_200, Color::from_rgba32_u32(0x2a2a2aff));
    env.set(NEUTRAL_300, Color::from_rgba32_u32(0x474747ff));
    env.set(NEUTRAL_400, Color::from_rgba32_u32(0x636363ff));
    env.set(NEUTRAL_500, Color::from_rgba32_u32(0x808080ff));
    env.set(NEUTRAL_600, Color::from_rgba32_u32(0x9c9c9cff));
    env.set(NEUTRAL_700, Color::from_rgba32_u32(0xb8b8b8ff));
    env.set(NEUTRAL_800, Color::from_rgba32_u32(0xd4d4d4ff));
    env.set(NEUTRAL_900, Color::from_rgba32_u32(0xf1f1f1ff));
    env.set(NEUTRAL_1000, Color::from_rgba32_u32(0xffffffff));

    // FIXME: this doesn't check if the font exists. probably not a huge deal rn
    // since it should just fall back to the default gtk font if it can't find roboto
    let font_family: FontFamily = FontFamily::new_unchecked("Roboto Flex");
    env.set(FONT, FontDescriptor::new(font_family));

    env.set(WINDOW_BACKGROUND_COLOR, env.get(NEUTRAL_1000));
    env.set(TEXT_COLOR, env.get(NEUTRAL_0));
    env.set(UI_FONT, env.get(FONT));
}
