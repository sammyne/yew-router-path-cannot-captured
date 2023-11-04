use yew::prelude::*;
use yew_router::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    yew::Renderer::<App>::new().render();
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/post/:id")]
    Post { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
fn App() -> Html {
    log::debug!("app");
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch_main} />
        </BrowserRouter>
    }
}

fn switch_main(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Post { id } => post(id),
        Route::NotFound => not_found(),
    }
}

#[function_component(Home)]
fn home() -> Html {
    log::debug!("home");
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn not_found() -> Html {
    html! {<h1>{"Not Found"}</h1>}
}

fn post(id: String) -> Html {
    log::debug!("post id={id}");
    html! {<p>{format!("You are looking at Post {}", id)}</p>}
}
