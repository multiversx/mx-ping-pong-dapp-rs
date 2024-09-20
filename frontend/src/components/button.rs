use yew::html::Classes;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub icon: Option<IconId>,
    pub icon_class: Option<String>,
    pub text_content: Option<String>,
    pub class_name: String,
    pub button_type: String,
    pub on_click: Callback<MouseEvent>,
    pub disabled: Option<bool>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button class={props.class_name.clone()} type={props.button_type.clone()} onclick={props.on_click.clone()} disabled={props.disabled.unwrap_or(false)}>
            if let Some(icon) = &props.icon {
                <Icon class={Classes::from(props.icon_class.clone())} icon_id={*icon} />
            }

            if let Some(content) = &props.text_content {
                { content }
            }
        </button>
    }
}
