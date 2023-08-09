use yew::prelude::*;
use crate::UserContext;
use crate::UserInfo;

use gloo_console::log;

#[function_component(Projects)]
pub fn app() ->Html {
  let user_context: UseReducerHandle<crate::UserInfo> = use_context::<UserContext>().unwrap();
  use_effect(move || {
    let info: UserInfo = UserInfo {
      page: "Projects".to_string(),
      dark_mode: user_context.dark_mode.to_owned(),
    };
    user_context.dispatch(info);
  });

  html! {
    <div class="flex justify-center items-center flex-1">
      {"Projects"}
    </div>
  }
}