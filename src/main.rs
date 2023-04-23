use yew::prelude::*;

#[function_component(TestExample)]
pub fn test_example() -> Html {
    let text = "Test Example";

    html! {
        <div>
            { text }
        </div>
    }
}

#[function_component]
fn App() -> Html {
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
            <button {onclick}>{"+1"}</button>
            <p>{ *counter }</p>
            <TestExample />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
