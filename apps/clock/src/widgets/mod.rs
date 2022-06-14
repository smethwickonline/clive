use std::time::Duration;

use druid::{widget::Label, TimerToken, Widget, Event, EventCtx, Env};
use crate::timekeeping::{Time};

pub struct ClockWidget {
    pub timer_id: TimerToken,
    pub label: Label<Time>,
}

static TIMER_INTERVAL: Duration = Duration::from_millis(100);

impl Widget<Time> for ClockWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Time, env: &Env) {
        match event {
            Event::WindowConnected => {
                self.timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    self.label.set_text(data.set_time().to_string());
                    ctx.request_layout();
                    self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                }
            }
            _ => (),
        }
        self.label.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &Time, env: &Env) {
        self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &Time, data: &Time, env: &Env) {
        self.label.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &Time, env: &Env) -> druid::Size {
        self.label.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Time, env: &Env) {
        self.label.paint(ctx, data, env)
    }
}