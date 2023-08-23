use crate::UserContext;
use crate::UserInfo;
use crate::router::Routes;
use crate::components::util;

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Toolbar)]
pub fn app() -> Html {
  let user_context: UseReducerHandle<UserInfo> = use_context::<UserContext>().unwrap();
  let dark_mode: bool = user_context.dark_mode;
  let user_context_clone = user_context.clone();
  let page_val: String = user_context.page.to_owned();

  // Handles the navigation on the toolbar
  let onclick_toggle_menu: Callback<MouseEvent> = Callback::from(move |_event: MouseEvent| {
    let info: crate::UserInfo = crate::UserInfo {
      page: user_context.page.to_owned(),
      dark_mode: user_context.dark_mode.to_owned(),
      toggle_dropdown: true
    };
    user_context.dispatch(info);
  });

  html! {
    <nav class="flex items-center h-12 mx-2 my-4">
      <Link<Routes> to={Routes::Main} classes="h-full">
        <button class="h-full" id="Home">
          <div class="flex items-center justify-center mx-8 h-full">
          <img src="../img/logo.png" alt="place holder" class={classes!(util::util::either!(user_context_clone.dark_mode == true => "invert"; "invert-0"), "h-10", "w-10", "object-cover")} />
            <div class="pl-4">
              {"Eric Xie"}
            </div>
          </div>
        </button>
      </Link<Routes>>

      <div class=" flex-grow" />

      <Link<Routes> to={Routes::About}>
        <button class={classes!("nav-item", "hidden", "ss:block", util::util::either!(page_val == "About" => "active-nav-item"; ""), util::util::either!(dark_mode == true => "hover:bg-dg-Light_Gray"; "hover:bg-lg-Light_Gray"))} id="About">
          {"About"}
        </button>
      </Link<Routes>>

      <Link<Routes> to={Routes::Projects{project: "~".to_string()}}>
        <button class={classes!("nav-item", "hidden", "ss:block", util::util::either!(page_val == "Projects" => "active-nav-item"; ""), util::util::either!(dark_mode == true => "hover:bg-dg-Light_Gray"; "hover:bg-lg-Light_Gray"))} id="Projects">
          {"Projects"}
        </button>
      </Link<Routes>>

      <Link<Routes> to={Routes::Contact}>
        <button class={classes!("nav-item", "mr-10", "hidden", "ss:block", util::util::either!(page_val == "Contact" => "active-nav-item"; ""), util::util::either!(dark_mode == true => "hover:bg-dg-Light_Gray"; "hover:bg-lg-Light_Gray"))} id="Contact">
          {"Contact"}
        </button>
      </Link<Routes>>

      <button class="h-10 w-10 flex flex-col divide-y ss:hidden" onclick={onclick_toggle_menu}>
        <img src="../img/ham_menu.png" alt="" class={classes!(util::util::either!(user_context_clone.dark_mode == true => "invert"; "invert-0"), "object-contain", "h-full", "w-full")} />
      </button>
    </nav>
  }
}

