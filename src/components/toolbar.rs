use yew::prelude::*;

#[function_component(Toolbar)]
fn app() -> Html {
    html! {
      <nav class="mx-2 px-2 flex items-center max-h-12">
        <button class="h-full">
          <div class="flex items-center">
            <img src="/img/github-mark.png" alt="place holder" class="h-10 object-cover pl-8" />
            <div class="pl-4">
              {"Eric Xie"}
            </div>
          </div>
        </button>
        <button class=" flex-grow" />
        <div class={classes!("nav-item", either!(*active_val == "About" => "active-nav-item"; ""))} id="About" onclick={onclick.clone()}>
          {"About"}
        </div>
        <button class={classes!("nav-item", either!(*active_val == "Projects" => "active-nav-item"; ""))} id="Projects" onclick={onclick.clone()}>
          {"Projects"}
        </button>
        <button class={classes!("nav-item", either!(*active_val == "Contact" => "active-nav-item"; ""))} id="Contact" onclick={onclick}>
          {"Contact"}
        </button>
        <a href="https://github.com/EJX537" target="_blank">
          <button class="h-full font-sans mx-2">
            <img src="/img/github-mark.png" alt="Github icon" class="h-10 object-cover" />
          </button>
        </a>
        <a href="https://www.linkedin.com/in/ericjxie/" target="_blank">
          <button class="h-full ml-2">
              <img src="/img/Linkedin.png" alt="Linkedin icon" class="h-10 object-cover" />
          </button>   
        </a>
      </nav>
    }
}
