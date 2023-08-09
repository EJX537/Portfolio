use yew::prelude::*;
use wasm_bindgen::JsCast;
use yew_router::prelude::*;

use crate::router::Routes;
use crate::{components::util::{self}, UserContext};

// use serde_json::to_string;


#[function_component(Toolbar)]
pub fn app() -> Html {
  let user_context: UseReducerHandle<crate::UserInfo> = use_context::<UserContext>().unwrap();
  let page_val: String = user_context.page.to_owned();

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
    <nav class="flex items-center max-h-12 mx-2 mb-3">
      <Link<Routes> to={Routes::Main}>
        <button class="h-full" id="Home" onclick={onclick_nav.clone()}>
          <div class="flex items-center justify-center mx-2 h-full">
          <img src="/img/github-mark.png" alt="place holder" class="h-10 object-cover" />
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
        <button class={classes!("nav-item", util::util::either!(page_val == "Contact" => "active-nav-item"; ""))} id="Contact" onclick={onclick_nav}>
          {"Contact"}
        </button>
      </Link<Routes>>

      <a href="https://github.com/EJX537" target="_blank" class="items-center justify-center flex">
        <button class="font-sans mx-2">
          <img src="/img/github-mark.png" alt="Github icon" class="h-10 object-cover" />
        </button>
      </a>

      <a href="https://www.linkedin.com/in/ericjxie/" target="_blank" class="items-center justify-center flex">
        <button class="ml-2">
            <img src="/img/Linkedin.png" alt="Linkedin icon" class="h-10 object-cover items-center justify-center" />
        </button>   
      </a>
    </nav>
  }
}

