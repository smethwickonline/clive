use crate::theme;
use druid::{
    widget::{Button, Flex, Container, Padding},
    Widget, WidgetExt,
};

fn action_bar(actions: [Button<String>; 3]) -> impl Widget<String> {
    let mut row = Flex::row();
    for action in actions {
        row.add_child(action);
    }
    let bar = Container::new(row).background(theme::THEME_EXTRA_LIGHT).padding((18.0, 12.0));
    bar
}
