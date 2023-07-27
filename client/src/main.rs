use yew::prelude::*;
use yew_router::prelude::*;

mod services;
use services::routing::{switch, Route};
mod views;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
