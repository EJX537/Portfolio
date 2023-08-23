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
  let user_context_clone2 = user_context.clone();
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

  let onclick_darkmode = Callback::from(move |_e: MouseEvent| {
    let info: crate::UserInfo = crate::UserInfo {
      page: user_context_clone2.page.to_owned(),
      dark_mode: !user_context_clone2.dark_mode.to_owned(),
      toggle_dropdown: user_context_clone2.toggle_dropdown.to_owned()
    };
    user_context_clone2.dispatch(info);
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
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6 mr-2 hover:cursor-pointer hidden sm:block" onclick={onclick_darkmode}>
        <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 009.002-5.998z" class={util::util::either!(dark_mode == true => "block"; "hidden")}/>
        <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12 18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z" class={util::util::either!(dark_mode == true => "hidden"; "block")} />
      </svg>

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

