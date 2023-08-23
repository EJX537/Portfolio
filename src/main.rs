mod components;
mod router;

use crate::components::util;
use crate::router::{switch, Routes};
use crate::components::toolbar::Toolbar;
use crate::components::sidebar::Sidebar;

use yew::prelude::*;
use yew::functional::use_reducer;
use std::rc::Rc;
use yew_router::prelude::*;

use gloo_console::log;

#[derive(Debug, PartialEq, Clone)]
pub struct UserInfo {
    pub page: String,
    pub dark_mode: bool,
    pub toggle_dropdown: bool
}

impl Reducible for UserInfo {
    type Action = UserInfo;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        UserInfo { page: action.page, dark_mode: action.dark_mode, toggle_dropdown: action.toggle_dropdown}.into()
    }
}

pub type UserContext = UseReducerHandle<UserInfo>;



#[function_component(App)]
pub fn app() -> Html {
    // States for the website
    // Window Size
    let user_context: UseReducerHandle<UserInfo> = use_reducer(|| UserInfo {
        page: "Home".to_string(),
        dark_mode: true,
        toggle_dropdown: false
    });

    html! {
    <body class={classes!(util::util::either!(user_context.dark_mode == true => "bg-dg-Cultured text-lg-Light_Gray"; "bg-lg-Cultured text-black"), "h-screen", "w-screen", "font-mono", "flex", "flex-col", "subpixel-antialiased")}>
        <ContextProvider<UserContext> context={user_context.clone()}>
            <BrowserRouter>
                <Toolbar />
                <Sidebar />
                <Switch<Routes> render={switch} />
            </BrowserRouter>
        </ContextProvider<UserContext>>
    </body>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}





