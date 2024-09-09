use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!("container")}>
            <div>{"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"}</div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
