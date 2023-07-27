use yew::prelude::*;
use yew_router::prelude::*;

use crate::views;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/button")]
    Button,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Button => html! { <views::ButtonPage /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
