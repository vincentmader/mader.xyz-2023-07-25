use yew::prelude::*;
use yew_router::prelude::*;

mod routing;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<routing::Route> render={routing::switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
