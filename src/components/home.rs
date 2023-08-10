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
  use_effect(move || {
    let info: UserInfo = UserInfo {
      page: "Home".to_string(),
      dark_mode: user_context.clone().dark_mode.to_owned(),
    };
    user_context.dispatch(info);
  });

  let window: Window = web_sys::window().unwrap();
  let to_nav: UseStateHandle<bool> = use_state(|| false);

  if *to_nav {
    let _ = window.location().replace("projects");
  }
  let counter: i32 = 0;
  util::onwheel(&window, counter, to_nav);

  html! {
    <div class="flex-1 flex-col flex">
    
      <div class="items-center justify-center grid grid-cols-2 mt-auto mb-4">
        <div class="justify-center items-center flex">
          { "Home" }
        </div>
        <div class="items-center justify-center flex">
          <img src="/img/bigtree.jpg" alt="place holder" class="sm:w-1 lg:w-64 2xl:w-96 " />
        </div>
      </div>

      <Link<Routes> to={Routes::Projects} classes="justify-center text-center mt-auto mb-4">
        <button class="items-center flex-col">
            {"Projects"}
            <div class="h-0 w-0 border-x-8 border-x-transparent border-b-[16px] border-b-black rotate-180 mx-auto"/>
        </button>
      </Link<Routes>>
    </div>
  }
}


