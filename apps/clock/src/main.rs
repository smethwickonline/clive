#![windows_subsystem = "windows"]

mod test;
mod timekeeping;
mod widgets;

use druid::{
    widget::{Flex, Label},
    AppLauncher, LocalizedString, Size, TimerToken, UnitPoint, WidgetExt, WindowDesc,
};
use stuff::ui::icon;
use timekeeping::Time;
use widgets::ClockWidget;

fn main() {
    let window = WindowDesc::new(
        Flex::column()
            .with_child(ClockWidget {
                timer_id: TimerToken::INVALID,
                label: Label::new("00:00:00").with_text_size(32f64),
            })
            .with_child(icon::icon())
            .align_vertical(UnitPoint::CENTER),
    )
    .window_size(Size::new(360.0, 640.0))
    .resizable(false)
    .title(LocalizedString::new("clock-window-title").with_placeholder("Clock"));

    let time = Time::new();

    AppLauncher::with_window(window)
        .log_to_console()
        .configure_env(|env, _| stuff::theme::setup(env))
        .launch(time)
        .expect("oh dear!");
}
