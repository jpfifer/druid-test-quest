use druid::{TimerToken, Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, WidgetPod};
use crate::{AppData, GameState};
use std::time::Duration;

pub struct TimerWidget<T> {
    timer_id: TimerToken,
    close_requested: bool,
    inner: WidgetPod<AppData, T>
}

impl<T: Widget<AppData>> TimerWidget<T> {
    pub fn new(widget: T) -> Self {
        Self {
            timer_id: TimerToken::INVALID,
            close_requested: false,
            inner: WidgetPod::new(widget)
        }
    }
}

const TIMER_INTERVAL: Duration = Duration::from_millis(200);

impl<T: Widget<AppData>> Widget<AppData> for TimerWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        match event {
            Event::WindowConnected => {
                self.timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Timer(id) => {
                if !self.close_requested {
                    if *id == self.timer_id {
                        if data.is_running() {
                            data.tick();
                        }
                        self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                    }
                }

            }
            Event::WindowCloseRequested => {
                self.close_requested = true;
            }
            _ => ()
        }
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppData, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppData, data: &AppData, env: &Env) {

        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppData, env: &Env) -> Size {
        let size = self.inner.layout(ctx, &bc.loosen(), data, env);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}

