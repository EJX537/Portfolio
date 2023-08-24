use crate::UserContext;
use crate::UserInfo;
use crate::router::Routes;
use crate::components::util;

use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::Window;

#[function_component(Main)]
pub fn app() ->Html {
  let user_context: UseReducerHandle<UserInfo> = use_context::<UserContext>().unwrap();
  let user_context_clone: UseReducerHandle<UserInfo> = user_context.clone();
  use_effect(move || {
    let info: UserInfo = UserInfo {
      page: "Home".to_string(),
      dark_mode: user_context.dark_mode.to_owned(),
      toggle_dropdown: user_context.toggle_dropdown.to_owned()
    };
    user_context.dispatch(info);
  });

  // Window Size

  let window: Window = web_sys::window().unwrap();
  let to_nav: UseStateHandle<bool> = use_state(|| false);
  let navigator: Navigator = use_navigator().unwrap();
  if *to_nav {
    navigator.push(&Routes::Projects{project: "~".to_string()});
  }
  let counter: i32 = 0;
  util::onwheel(&window, counter, to_nav);

  html! {
    <div class="flex-1 flex-col flex h-full w-full text-inherit">
      <div class="flex flex-1 justify-center items-center flex-col p-2">
        <div class="justify-center flex flex-col font">
          <span class="font-bold p-2 text-lg">
            { "Hello, I'm Eric."}
          </span>
          <span class={classes!(util::util::either!(user_context_clone.dark_mode == true => "bg-black text-lg-Cultured"; "bg-white text-dg-Cultured"), "font-black", "p-2", "rounded-sm", "text-xl")}>
            {"I am a software developer." }
          </span>
        </div>
        <img src="../img/logo.png" alt="" class={classes!(util::util::either!(user_context_clone.dark_mode == true => "invert"; "invert-0"), "w-3/5", "mt-2", "p-2", "max-h-[500px]", "object-contain")} />
      </div>

      <Link<Routes> to={Routes::Projects{project: "~".to_string()}} classes="justify-center text-center mt-auto mb-4">
        <button class="items-center flex-col">
            {"Projects"}
            <div class={classes!(util::util::either!(user_context_clone.dark_mode == true => "border-b-lg-Cultured"; "border-b-dg-Cultured"), "h-0", "w-0", "border-x-8", "border-x-transparent", "border-b-[16px]", "rotate-180", "mx-auto")}/>
        </button>
      </Link<Routes>>
    </div>
  }
}


