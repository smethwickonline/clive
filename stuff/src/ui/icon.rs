#![allow(dead_code)] // TODO: remove this before pushing or be eternally embarrased
use druid::{widget::{Svg, SvgData, FillStrat, Flex}, Widget, Data, WidgetExt};

/// Helper widget for drawing icons from the Phosphor set.
struct IconWidget {
    icon: Icon,
    svg: Svg,
}

/// it's an icon. you can find the names of all the icons
/// at https://phosphoricons.com/
#[derive(Data, Clone)]
pub struct Icon {
    name: String,
    #[data(eq)]
    weight: IconWeight,
}

#[derive(Clone, PartialEq)]
pub enum IconWeight {
    Thin,
    Light,
    Regular,
    Bold,
    Fill,
    Duotone,
}

pub fn icon<T: druid::Data>() -> impl Widget<T> {
    let svg = match include_str!("./icons/test.svg").parse::<SvgData>() {
        Ok(svg) => svg,
        Err(err) => {
            println!("{}", err);
            SvgData::default()
        }
    };

    let svg = Svg::new(svg.clone()).fix_width(32.0).center();
    svg
}

