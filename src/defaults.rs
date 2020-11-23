//
use legion::*;
use acute_app::{Timer, Plugin};

#[system]
pub fn update_timer(#[resource] timer: &mut Timer) {
    timer.update_delta_time();
    timer.update_fixed_time();
}

pub trait DefaultAddons {
    fn with_defaults(&mut self) -> &mut Self;
    fn with_defaults_headless(&mut self) -> &mut Self;
}

impl DefaultAddons for acute_app::AppBuilder {
    fn with_defaults(&mut self) -> &mut Self {
        let window_plugin = acute_window::WinitWindowPlugin::default();
        window_plugin.add(self);
        // let render_plugin = acute_render_backend::WGPURenderPlugin::default();
        // render_plugin.add(self);

        self
    }

    fn with_defaults_headless(&mut self) -> &mut Self {
        self.add_resource(Timer::new())
            .add_system(update_timer_system())
    }
}
