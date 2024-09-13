use crate::components::Button;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct TxStatusModalProps {
    pub status: String,
    pub children: Children,
    pub on_close: Callback<MouseEvent>,
    pub is_visible: bool,
}

#[function_component(TxStatusModal)]
pub fn tx_status_modal(props: &TxStatusModalProps) -> Html {
    let visibility_class = if props.is_visible { "show" } else { "hide" };
    html! {
        <div id = "txStatusModal" class={visibility_class}>
            <Button class_name="close-btn" button_type="button" on_click={props.on_close.clone()}>
                <Icon class="iconClose" icon_id={IconId::FontAwesomeSolidXmark}/>
            </Button>
            <h2>{ "Transaction Status" }</h2>
            <div id="statusContainer">
                { for props.children.iter() }
                <h3 id="status">{ props.status.clone() }</h3>
            </div>
        </div>
    }
}
