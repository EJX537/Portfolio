use yew::prelude::*;
use crate::UserContext;
use crate::UserInfo;



#[function_component(About)]
pub fn app() -> Html {
  let user_context: UseReducerHandle<crate::UserInfo> = use_context::<UserContext>().unwrap();
  use_effect(move || {
    let info: UserInfo = UserInfo {
      page: "About".to_string(),
      dark_mode: user_context.dark_mode.to_owned(),
    };
    user_context.dispatch(info);
  });
  html! {
    <div class="flex justify-center items-center flex-1">
      {"About"}
    </div>
  }
}



