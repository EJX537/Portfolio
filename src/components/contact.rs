use crate::UserContext;
use crate::UserInfo;

use yew::prelude::*;

#[function_component(Contact)]
pub fn app() -> Html {
  let user_context: UseReducerHandle<crate::UserInfo> = use_context::<UserContext>().unwrap();
  use_effect(move || {
    let info: UserInfo = UserInfo {
      page: "Contact".to_string(),
      dark_mode: user_context.dark_mode.to_owned(),
    };
    user_context.dispatch(info);
  });

  html! {
    <div class="flex-1 flex flex-row">
      <div class="contact-box group">
        <div class="flex justify-center group-hover:font-bold group-hover:underline underline-offset-4">
          {"Peronsal Contact"}
        </div>
        <div class="flex justify-center mt-6">
          <div class="mr-2">
            {"Personal Email: "}
          </div>
          <a href="mailto:meisericxie@gmail.com?subject=Hello&body=" target="_blank" class="items-center justify-center flex hover:underline">
            <button>
              {"meisericxie@gmail.com"}
            </button>
          </a>
        </div>
        <div class="flex justify-center mt-6">
          <div class="mr-2">
            {"Discord: "}
          </div>
          <a href="https://discord.com/channels/290328963488940034" target="_blank" class="items-center justify-center flex hover:underline">
            <button>
              {"This is Who?"}
            </button>
          </a>
        </div>
      </div>
      <div class="contact-box group">
        <div class="flex justify-center group-hover:font-bold group-hover:underline underline-offset-4">
          {"Social Media"}
        </div>
        <div class="flex justify-center mt-6">
          {"Don't use"}
        </div>
      </div>
    </div>
  }
}



