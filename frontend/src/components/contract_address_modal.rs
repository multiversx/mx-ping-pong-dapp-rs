use crate::components::Button;
use _ContractAddressModalProps::address;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct ContractAddressModalProps {
    pub address: String,
    pub is_extended: bool,
    pub on_extend: Callback<MouseEvent>,
    pub arrow_id: IconId,
}

#[function_component(ContractAddressModal)]
pub fn contract_address_modal(props: &ContractAddressModalProps) -> Html {
    let visibility_class_name = if !props.is_extended {
        "compressed"
    } else {
        "extended"
    };

    let (explorer_link, redirect_visible) = if props.address.starts_with("erd") {
        (
            format!("https://explorer.multiversx.com/accounts/{}", props.address),
            "visible".to_string(),
        )
    } else {
        ("#".to_string(), "hidden".to_string())
    };

    html! {
        <div id="contractAddrModal" class={visibility_class_name}>
            <div id="addrHeaderContainer">
                <Button class_name="toggleButton" button_type="button" on_click={props.on_extend.clone()}>
                    <Icon class="toggleArrow" icon_id={props.arrow_id}/>
                </Button>
                <h2>{ "Contract Address" }</h2>
                <a
                    id="redirectLink"
                    target="_blank"
                    href={explorer_link}
                    title="MultiversX Explorer"
                    >
                    <Icon class={classes!(redirect_visible, "redirectIcon")} icon_id={IconId::OcticonsLinkExternal16}/>
                </a>
            </div>
            <div id="addrContainer">
                if props.address.is_empty() {
                    <p id="contractAddr">{ "No contract deployed" }</p>
                } else {
                    <p id="contractAddr">{ &props.address }</p>
                }
            </div>
        </div>

    }
}
