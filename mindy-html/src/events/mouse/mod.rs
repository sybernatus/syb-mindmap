use crate::{SHEET_POSITION, SHEET_ZOOM};
use dioxus::logger::tracing;
use dioxus::prelude::*;

pub fn mouse_data_update(
    mut is_dragging: Signal<bool>,
    mut last_mouse: Signal<(f64, f64)>,
) -> impl Fn(Event<MouseData>) {
    move |event: Event<MouseData>| {
        use_future(move || {
            let value = event.clone();
            async move {
                tracing::trace!("Mouse down event: {:?}", value);
                is_dragging.set(true);
                last_mouse.set((
                    value.data().coordinates().client().x,
                    value.data().coordinates().client().y,
                ));
                tracing::trace!("Mouse down position: {:?}", last_mouse);
            }
        });
    }
}

pub fn mouse_position_update(
    is_dragging: Signal<bool>,
    mut last_mouse: Signal<(f64, f64)>,
) -> impl Fn(Event<MouseData>) {
    move |event: Event<MouseData>| {
        use_future(move || {
            let value = event.clone();
            async move {
                if is_dragging() {
                    let current_mouse = (
                        value.data.coordinates().client().x,
                        value.data.coordinates().client().y,
                    );
                    let dx = (current_mouse.0 - last_mouse().0) / SHEET_ZOOM();
                    let dy = (current_mouse.1 - last_mouse().1) / SHEET_ZOOM();
                    *SHEET_POSITION.write() = (
                        SHEET_POSITION().0 + dx,
                        SHEET_POSITION().1 + dy,
                    );
                    last_mouse.set(current_mouse);
                }
            }
        });
    }
}

pub fn mouse_dragging_disable(mut is_dragging: Signal<bool>) -> impl Fn(Event<MouseData>) {
    move |_event: Event<MouseData>| {
        use_future(move || async move {
            is_dragging.set(false);
        });
    }
}



pub fn mouse_zooming_update() -> impl Fn(Event<WheelData>) {
    move |event: Event<WheelData>| {
        use_future(move || {
            let value = event.clone();
            async move {
                if SHEET_ZOOM() < 0.01 {
                    *SHEET_ZOOM.write() = 0.01;
                } else {
                    let scale = SHEET_ZOOM() + -value.data.delta().strip_units().y.clamp(-0.1, 0.1);
                    *SHEET_ZOOM.write() = scale;
                }
            }
        });
    }
}
