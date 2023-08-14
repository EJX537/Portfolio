mod components;
mod router;

use crate::components::util::use_window_size;
use crate::router::{switch, Routes};
use crate::components::toolbar::Toolbar;

use yew::prelude::*;
use yew::functional::use_reducer;
use std::rc::Rc;
use yew_router::prelude::*;

use gloo_console::log;

#[derive(Debug, PartialEq, Clone)]
pub struct UserInfo {
    pub page: String,
    pub dark_mode: bool,
}

impl Reducible for UserInfo {
    type Action = UserInfo;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        UserInfo { page: action.page, dark_mode: action.dark_mode}.into()
    }
}

pub type UserContext = UseReducerHandle<UserInfo>;



#[function_component(App)]
pub fn app() -> Html {
    // States for the website
    // Window Size
    let user_context: UseReducerHandle<UserInfo> = use_reducer(|| UserInfo {
        page: "Home".to_string(),
        dark_mode: false,
    });


    html! {
    <body class="h-screen w-screen sm:bg-lg-Cultured bg-blue-50 font-mono flex flex-col pt-3 subpixel-antialiased">
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

