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
  let user_context_clone2: UseReducerHandle<UserInfo> = user_context.clone();

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

  let onclick_darkmode = Callback::from(move |_e: MouseEvent| {
    let info: crate::UserInfo = crate::UserInfo {
      page: user_context_clone2.page.to_owned(),
      dark_mode: !user_context_clone2.dark_mode.to_owned(),
      toggle_dropdown: user_context_clone2.toggle_dropdown.to_owned()
    };
    user_context_clone2.dispatch(info);
  });

  html! {
    <div class={classes!(util::util::either!(user_context_clone.toggle_dropdown == true => "block"; "hidden"),
      util::util::either!(user_context_clone.dark_mode == true => "bg-black"; "bg-white"), "absolute", "overflow-hidden",
        "right-0", "h-screen", "w-screen", "z-50", "shadow-lg", "bg-opacity-75")} onclick={onclick_close} id="closable" >
        
      <ul class={classes!(util::util::either!(user_context_clone.dark_mode == true => "bg-dg-Gainsboro divide-lg-Cultured"; "bg-lg-Gainsboro divide-dg-Cultured"),
        "absolute", "h-full", "w-4/5", "divide-y-2", "ml-auto", "p-4", "rounded-l-lg", "rounded-bl-lg", "pt-[50%]", "items-center", "text-xl",
          "transition-transform", "-right-80", "-translate-x-80", "duration-300")}>

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
            <button class="" id="closable">
                {"Contact"}
            </button>
        </Link<Routes>>
        </li>
        <li class="py-4 justify-center">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-10 h-10 mr-2 hover:cursor-pointer" onclick={onclick_darkmode}>
            <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 009.002-5.998z" class={util::util::either!(user_context_clone.dark_mode == true => "block"; "hidden")}/>
            <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12 18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z" class={util::util::either!(user_context_clone.dark_mode == true => "hidden"; "block")} />
          </svg>
        </li>
      </ul>
    </div>
  }
}




