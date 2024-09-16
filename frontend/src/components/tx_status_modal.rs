use crate::components::Button;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct TxStatusModalProps {
    pub status: String,
    pub icon: IconId,
    pub on_close: Callback<MouseEvent>,
    pub is_visible: bool,
}

#[function_component(TxStatusModal)]
pub fn tx_status_modal(props: &TxStatusModalProps) -> Html {
    let visibility_class = if props.is_visible { "show" } else { "hide" };
    html! {
        <div id = "txStatusModal" class={visibility_class}>
            <Button class_name="close-btn" button_type="button" on_click={props.on_close.clone()} icon={IconId::FontAwesomeSolidXmark} icon_class={"icon-close".to_string()} />
            <h2>{ "Transaction Status" }</h2>
            <div id="statusContainer">
                <Icon class="icon-status" icon_id={props.icon} />
                <h3 id="status">{ props.status.clone() }</h3>
            </div>
        </div>
    }
}
