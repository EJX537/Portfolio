use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::Window;
use web_sys::window;

#[derive(Debug, Clone, PartialEq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32
}

pub mod util {
    macro_rules! either {
        ($test:expr => $true_expr:expr; $false_expr:expr) => {
            if $test {
                $true_expr
            }
            else {
                $false_expr
            }
        }
      }
    pub(crate) use either;
}


pub fn onwheel(window: &Window, mut counter: i32, to_nav: UseStateHandle<bool>) {
    let closure: Closure<dyn FnMut(WheelEvent)> = Closure::wrap(Box::new(move |event: WheelEvent| {
        if event.delta_y() > 0.0 {
            counter += 1;
            if counter == 1 {
                to_nav.set(true);
            }
        } else {
            counter = 0;
        }
    }) as Box<dyn FnMut(WheelEvent)>);
    
    window.set_onwheel(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

#[hook]
pub fn use_window_size() -> UseStateHandle<WindowSize> {
    let window_size: UseStateHandle<WindowSize> = use_state(|| WindowSize {
        width: window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32,
        height: window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32
    });


    let resize = {
        let set_window_size = window_size.clone();
        move || {
            let window_size = WindowSize {
                width: window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32,
                height: window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32
            };
            set_window_size.set(window_size);
        }
    };

    use_effect_with_deps(
        move |_| {
            let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(resize) as Box<dyn FnMut()>);
            window()
                .unwrap()
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            let closure = closure.into_js_value();
            move || {
                window()
                    .unwrap()
                    .remove_event_listener_with_callback("resize", closure.unchecked_ref())
                    .unwrap();
            }
        },
        (),
    );

    window_size
}
