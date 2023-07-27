use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, yew::Properties, std::default::Default)]
pub struct Props {}

#[function_component(ButtonPage)]
pub fn button_page(_props: &Props) -> Html {
    let counter = use_state(|| 0);
    let onclick_1 = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let navigator = use_navigator().unwrap();
    let onclick_2 = Callback::from(move |_| navigator.push(&crate::routing::Route::Home));

    html! {
        <div>
            <button onclick={onclick_1}>{ "+1" }</button>
            <p>{ *counter }</p>
            <button onclick={onclick_2}>{ "Go Home" }</button>
        </div>
    }
}
