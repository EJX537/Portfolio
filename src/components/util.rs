use yew::prelude::*;
use std::collections::HashMap;
use std::any::type_name;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::Window;
use gloo_console::log;
use std::rc::Rc;
use std::cell::RefCell;

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


pub fn onwheel(window: &Window, to_nav: UseStateHandle<bool>) {
    let counter: i32 = 0;
    let closure: Closure<dyn FnMut(WheelEvent)> = Closure::wrap(Box::new(move |event: WheelEvent| {
        if event.delta_y() > 0.0 {
            let mut counter_val: i32 = counter.clone();
            counter_val += 1;
            if counter_val >= 15 {
                to_nav.set(true);
            }
        }
    }) as Box<dyn FnMut(WheelEvent)>);

    let reset_closure = Closure::wrap(Box::new(move || {
        // let mut counter_value: i32 = *counter;
        // counter_value = 0;
    }) as Box<dyn Fn()>);

    let _timeout_id: Result<i32, wasm_bindgen::JsValue> = window.set_timeout_with_callback_and_timeout_and_arguments_0( reset_closure.as_ref().unchecked_ref()
        , 1000
    );
    window.set_onwheel(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    reset_closure.forget();
}
