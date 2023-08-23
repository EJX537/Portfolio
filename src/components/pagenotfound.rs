use crate::UserContext;
use crate::UserInfo;
use crate::router::Routes;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub path: String
}


#[function_component(Page404)]
pub fn app(props: &Props) ->Html {
  let user_context: UseReducerHandle<UserInfo> = use_context::<UserContext>().unwrap();
  let path: String = props.path.clone();
  let path_clone: String = props.path.clone();
  let navigator: Navigator = use_navigator().unwrap();

  use_effect(move || {
    if path == "projects" {
      let info: UserInfo = UserInfo {
        page: "Projects".to_string(),
        dark_mode: user_context.dark_mode.to_owned(),
        toggle_dropdown: user_context.toggle_dropdown.to_owned()
      };
      user_context.dispatch(info);
      navigator.push(&Routes::Projects{project: "~".to_string()});
    }

  });

  html! {
    <div class="flex justify-center items-center flex-1">
      {"Page "} {path_clone} {" Not Found"}
    </div>
  }
}
