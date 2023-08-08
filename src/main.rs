use yew::prelude::*;
use yew_router::prelude::*;
use yew::{classes, html};
use gloo_console::log;
use wasm_bindgen::{JsValue, JsCast};
use serde_json::to_string;

use std::any::type_name;
use std::borrow::Borrow;

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


// use yew_router::prelude::*;

fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}


#[function_component(App)]
fn app() -> Html {
    let active: UseStateHandle<String> = use_state(|| "".to_string());
    let active_val: UseStateHandle<String> = active.clone();
    let onclick: Callback<MouseEvent> = Callback::from(move |event: MouseEvent| {
        let target: web_sys::EventTarget = event.target().unwrap();
        let element: web_sys::HtmlElement = target.dyn_into::<web_sys::HtmlElement>().unwrap();
        let id: String = element.id();
        log!(JsValue::from(id.clone()));
        active.set(id);
    });

    html! {
    <body class="min-h-screen w-screen bg-gray-100 font-mono flex flex-col">
        <div class="h-2"/>

    </body>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}