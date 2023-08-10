mod components;
mod router;

use crate::router::{switch, Routes};
use crate::components::toolbar::Toolbar;

use yew::prelude::*;
use yew::functional::use_reducer;
use std::rc::Rc;
use yew_router::prelude::*;



// use wasm_bindgen::{JsValue, JsCast};

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
    html! {
    <body class="min-h-screen max-h-screen w-screen bg-lg-Cultured font-mono flex flex-col pt-3">
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