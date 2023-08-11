use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::JsCast;

use crate::router::Routes;
use crate::{components::util::{self}, UserContext};

// use serde_json::to_string;


#[function_component(Toolbar)]
pub fn app() -> Html {
  let user_context: UseReducerHandle<crate::UserInfo> = use_context::<UserContext>().unwrap();
  let page_val: String = user_context.page.to_owned();

  // Handles the navigation on the toolbar
  let onclick_nav: Callback<MouseEvent> = Callback::from(move |event: MouseEvent| {
    let target: web_sys::EventTarget = event.target().unwrap();
    let element: web_sys::HtmlElement = target.dyn_into::<web_sys::HtmlElement>().unwrap();
    let id: String = element.id();

    let info: crate::UserInfo = crate::UserInfo {
      page: id,
      dark_mode: user_context.dark_mode.to_owned(),
    };
    user_context.dispatch(info);
  });

  html! {
    <nav class="flex items-center max-h-12 min-h-12 mx-2">
      <Link<Routes> to={Routes::Main}>
        <button class="h-full" id="Home" onclick={onclick_nav.clone()}>
          <div class="flex items-center justify-center mx-8 h-full">
          <img src="" alt="place holder" class="h-10 w-10 object-cover" />
            <div class="pl-4">
              {"Eric Xie"}
            </div>
          </div>
        </button>
      </Link<Routes>>

      <div class=" flex-grow" />

      <Link<Routes> to={Routes::About}>
        <button class={classes!("nav-item", util::util::either!(page_val == "About" => "active-nav-item"; ""))} id="About" onclick={onclick_nav.clone()}>
          {"About"}
        </button>
      </Link<Routes>>

      <Link<Routes> to={Routes::Projects}>
        <button class={classes!("nav-item", util::util::either!(page_val == "Projects" => "active-nav-item"; ""))} id="Projects" onclick={onclick_nav.clone()}>
          {"Projects"}
        </button>
      </Link<Routes>>

      <Link<Routes> to={Routes::Contact}>
        <button class={classes!("nav-item", "mr-10", util::util::either!(page_val == "Contact" => "active-nav-item"; ""))} id="Contact" onclick={onclick_nav}>
          {"Contact"}
        </button>
      </Link<Routes>>
    </nav>
  }
}

