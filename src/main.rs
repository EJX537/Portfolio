use yew::prelude::*;
// use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <body class="min-h-screen w-screen bg-gray-100">
        <nav class="pt-4 px-2 flex items-center h-12">
            <div class="text-black h-full">
                <div class="h-full flex items-center">
                    <img src="/img/github-mark.png" alt="place holder" class="h-full object-cover pl-8" />
                <div class="pl-4">
                    {"Eric Xie"}
                </div>
                </div>
            </div>
            <div class=" flex-grow" />
            <button class="m-2">
                {"About"}
            </button>
            <button class="m-2">
                {"Projects"}
            </button>
            <div class="h-full mx-2">
                <img src="/img/github-mark.png" alt="Github icon" class="h-full object-cover" />
            </div>
            <div class="h-full px-2">
                <img src="/img/Linkedin.png" alt="Linkedin icon" class="h-full object-cover" />
            </div>
        </nav>
    </body>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}