use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Children,
    pub class_name: String,
    pub button_type: String,
    pub on_click: Callback<MouseEvent>,
    pub disabled: Option<bool>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button class={props.class_name.clone()} type={props.button_type.clone()} onclick={props.on_click.clone()} disabled={props.disabled.unwrap_or(false)}>
            { for props.children.iter() }
        </button>
    }
}
