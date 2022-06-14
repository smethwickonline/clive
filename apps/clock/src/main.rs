#![windows_subsystem = "windows"]

mod timekeeping;
mod widgets;
mod test;


use druid::{
    widget::{Flex, Label},
    AppLauncher, LocalizedString, Size, TimerToken, WindowDesc, UnitPoint, WidgetExt,
};
use timekeeping::Time;
use widgets::ClockWidget;

fn main() {
    let window = WindowDesc::new(
        Flex::column()
            .with_child(ClockWidget {
                timer_id: TimerToken::INVALID,
                label: Label::new("00:00:00").with_text_size(32f64),
            })
            .align_vertical(UnitPoint::CENTER),
    )
    .window_size(Size::new(360.0, 640.0))
    .resizable(false)
    .title(LocalizedString::new("clock-window-title").with_placeholder("Clock"));

    let time = Time::new();

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(time)
        .expect("oh dear!");
}
