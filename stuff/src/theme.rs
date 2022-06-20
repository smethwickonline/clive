use druid::{theme::*, FontDescriptor, FontFamily};
use druid::{Color, Env, Key};

pub const NEUTRAL_0: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_0");
pub const NEUTRAL_100: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_100");
pub const NEUTRAL_200: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_200");
pub const NEUTRAL_300: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_300");
pub const NEUTRAL_400: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_400");
pub const NEUTRAL_500: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_500");
pub const NEUTRAL_600: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_600");
pub const NEUTRAL_700: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_700");
pub const NEUTRAL_800: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_800");
pub const NEUTRAL_900: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_900");
pub const NEUTRAL_1000: Key<Color> = Key::new("online.smethwick.clive.theme.neutral_1000");

pub const THEME_EXTRA_LIGHT: Key<Color> = Key::new("online.smethwick.clive.theme.theme_el");
pub const THEME_LIGHT: Key<Color> = Key::new("online.smethwick.clive.theme.theme_l");
pub const THEME_BRIGHT: Key<Color> = Key::new("online.smethwick.clive.theme.theme_b");
pub const THEME_DARK: Key<Color> = Key::new("online.smethwick.clive.theme.theme_d");
pub const THEME_EXTRA_DARK: Key<Color> = Key::new("online.smethwick.clive.theme.theme_ed");

pub const FONT: Key<FontDescriptor> = Key::new("online.smethwick.clive.theme.ui_font");

pub fn set_theme_colours_based_on_the_theme_colour_that_is_currently_set_thanks(env: &mut Env) {
    // TODO: get theme from config db
    // "leah why don't you use an enum for this" why don't you shut the fuck up
    let theme = "ectoplasm";

    match theme {
        "ectoplasm" => {
            env.set(THEME_EXTRA_LIGHT, Color::from_rgba32_u32(0xcff6d3ff));
            env.set(THEME_LIGHT, Color::from_rgba32_u32(0xA4E9AFff));
            env.set(THEME_BRIGHT, Color::from_rgba32_u32(0x61C182ff));
            env.set(THEME_DARK, Color::from_rgba32_u32(0x0C2B15ff));
            env.set(THEME_EXTRA_DARK, Color::from_rgba32_u32(0x02140Aff));
        }
        "weezer" => {
            env.set(THEME_EXTRA_LIGHT, Color::from_rgba32_u32(0xE3F8FFff));
            env.set(THEME_LIGHT, Color::from_rgba32_u32(0x84D6F2ff));
            env.set(THEME_BRIGHT, Color::from_rgba32_u32(0x009ACEff));
            env.set(THEME_DARK, Color::from_rgba32_u32(0x09303Dff));
            env.set(THEME_EXTRA_DARK, Color::from_rgba32_u32(0x020F13ff));
        }
        &_ => {
            env.set(THEME_EXTRA_LIGHT, Color::from_rgba32_u32(0xcff6d3ff));
            env.set(THEME_LIGHT, Color::from_rgba32_u32(0xA4E9AFff));
            env.set(THEME_BRIGHT, Color::from_rgba32_u32(0x61C182ff));
            env.set(THEME_DARK, Color::from_rgba32_u32(0x0C2B15ff));
            env.set(THEME_EXTRA_DARK, Color::from_rgba32_u32(0x02140Aff));
        }
    }
}

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

    set_theme_colours_based_on_the_theme_colour_that_is_currently_set_thanks(env);

    // FIXME: this doesn't check if the font exists. probably not a huge deal rn
    // since it should just fall back to the default gtk font if it can't find roboto
    let font_family: FontFamily = FontFamily::new_unchecked("Roboto Flex");
    env.set(FONT, FontDescriptor::new(font_family));

    env.set(WINDOW_BACKGROUND_COLOR, env.get(NEUTRAL_1000));
    env.set(TEXT_COLOR, env.get(NEUTRAL_0));
    env.set(UI_FONT, env.get(FONT));
}
