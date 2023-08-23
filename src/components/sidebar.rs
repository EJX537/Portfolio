use crate::UserContext;
use crate::UserInfo;
use crate::router::Routes;
use crate::components::util;

use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::JsCast;

#[function_component(Sidebar)]
pub fn app() -> Html {
  let user_context: UseReducerHandle<UserInfo> = use_context::<UserContext>().unwrap();
  let user_context_clone: UseReducerHandle<UserInfo> = user_context.clone();
  let onclick_close: Callback<MouseEvent> = Callback::from(move |event: MouseEvent| {
    let target: web_sys::EventTarget = event.target().unwrap();
    let element: web_sys::HtmlElement = target.dyn_into::<web_sys::HtmlElement>().unwrap();
    let id: String = element.id();

    if id == "closable" {
      let info: UserInfo = UserInfo {
        page: user_context.page.to_owned(),
        dark_mode: user_context.dark_mode.to_owned(),
        toggle_dropdown: false
      };
      user_context.dispatch(info);
    }
  });

  let onclick_no = Callback::from(move |_e: MouseEvent| {

  });

  html! {
    <div class={classes!(util::util::either!(user_context_clone.toggle_dropdown == true => "block"; "hidden"), util::util::either!(user_context_clone.dark_mode == true => "bg-black"; "bg-white"), "absolute", "overflow-hidden", "right-0", "h-screen", "w-screen", "z-50", "shadow-lg", "bg-opacity-75")} onclick={onclick_close} id="closable">
      <ul class={classes!(util::util::either!(user_context_clone.dark_mode == true => "bg-dg-Gainsboro divide-lg-Cultured"; "bg-lg-Gainsboro divide-dg-Cultured"), "absolute", "h-full", "w-4/5", "divide-y-2", "ml-auto", "p-4", "rounded-l-lg", "rounded-bl-lg", "pt-[50%]", "items-center", "text-xl", "transition-transform", "-right-80", "-translate-x-80", "duration-300")}>
        <li class="justify-center py-4">
            <Link<Routes> to={Routes::About}>
                <button class="" id="closable">
                    {"About"}
                </button>
            </Link<Routes>>
        </li>
        <li class="justify-center py-4">
            <Link<Routes> to={Routes::Projects{project: "~".to_string()}}>
                <button class="" id="closable">
                    {"Projects"}
                </button>
            </Link<Routes>>
        </li>
        <li class="justify-center py-4">
            <Link<Routes> to={Routes::Contact}>
            <button class="" id="closable" onclick={onclick_no}>
                {"Contact"}
            </button>
        </Link<Routes>>
        </li>
      </ul>
    </div>
  }
}




