use yew::prelude::*;
use std::collections::HashMap;
use std::any::type_name;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::Window;
use gloo_console::log;

pub fn to_string<'a>(hmap: &'a HashMap<&'a str, &'a str>, key: &'a str) -> Option<&'a str> {
    if let Some(value) = hmap.get(key) {
        Some(*value)
    } else {
        None
    }
}

pub fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
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
            if counter > 2 {
                to_nav.set(true);
            }
        } else {
            counter = 0;
        }
        log!("1");
    }) as Box<dyn FnMut(WheelEvent)>);
    
    window.set_onwheel(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}



