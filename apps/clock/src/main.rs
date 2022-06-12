use chrono::{self, Local};
use druid::widget::Label;
use druid::{
    AppLauncher, LocalizedString, PlatformError, Size, Widget, WidgetExt, WindowDesc,
};
#[derive(Copy, Clone, Debug)]
struct AppData {
    current_time: chrono::DateTime<Local>,
}

fn main() -> Result<(), PlatformError> {
    let data = chrono::Local::now().to_string();
    let main_window = WindowDesc::new(ui_builder())
        .window_size(Size::new(360.0, 640.0))
        .resizable(false);
    AppLauncher::with_window(main_window)
        .log_to_console()
        .localization_resources(vec!["main.ftl".to_owned()], "strings".to_owned())
        .launch(data)
}

fn ui_builder() -> impl Widget<String> {
    let text = LocalizedString::new("test-string")
        .with_arg("thyme", |data: &String, _env| data.to_string().into());
    let label = Label::new(text).padding(5.0).center();

    label
}
