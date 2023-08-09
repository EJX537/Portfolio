mod components;
mod router;

use crate::router::{switch, Routes};
use crate::components::toolbar::Toolbar;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::Closure;

use web_sys::{Window, window};
use yew::prelude::*;
use yew::functional::use_reducer;
use std::cell::RefCell;
use std::rc::Rc;
use yew_router::prelude::*;
use crate::components::util;
use js_sys::Date;

use gloo_console::log;
use serde_json::to_string;

// use wasm_bindgen::{JsValue, JsCast};

// use std::borrow::Borrow;
// use std::string;


#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct UserInfo {
    pub page: String,
    pub dark_mode: bool,
}

impl Reducible for UserInfo {
    type Action = UserInfo;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        UserInfo { page: action.page, dark_mode: action.dark_mode }.into()
    }
}

pub type UserContext = UseReducerHandle<UserInfo>;

#[function_component(App)]
pub fn app() -> Html {
    // States for the website
    // Which page the user is on
    let user_context: UseReducerHandle<UserInfo> = use_reducer(|| UserInfo {
        page: "Home".to_string(),
        dark_mode: false,
    });
    let window: Window = web_sys::window().unwrap();
    let to_nav: UseStateHandle<bool> = use_state(|| false);

    use_effect(move || {
        log!(1);
        let now = Date::new_0();
        let year = now.get_full_year();
        let month = now.get_month() + 1;
        let day = now.get_date();
        log!(year);
      });

    if *to_nav {
        let page: String = user_context.page.to_owned();
        if page ==  "Home" {

            let _ = window.location().replace("about");
        } 
        if page == "About" {
            let _ = window.location().replace("projects");
        }
    }
    util::onwheel(&window, to_nav);
        
    html! {
    <body class="min-h-screen w-screen bg-gray-100 font-mono flex flex-col pt-3">
        <ContextProvider<UserContext> context={user_context}>
            <BrowserRouter>
                <Toolbar />
                <Switch<Routes> render={switch} />
            </BrowserRouter>
        </ContextProvider<UserContext>>
    </body>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}