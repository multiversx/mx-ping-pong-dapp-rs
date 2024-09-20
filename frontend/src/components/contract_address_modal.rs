use crate::components::Button;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

const DEVNET_ACCOUNTS_LINK: &str = "https://devnet-explorer.multiversx.com/accounts";

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
            format!("{}/{}", DEVNET_ACCOUNTS_LINK, props.address),
            "visible".to_string(),
        )
    } else {
        ("#".to_string(), "hidden".to_string())
    };

    html! {
        <div id="contractAddrModal" class={visibility_class_name}>
            <div id="addrHeaderContainer">
                <Button class_name="toggle-button" button_type="button" on_click={props.on_extend.clone()} icon={props.arrow_id} icon_class={"toggle-arrow".to_string()} />
                <h2>{ "Contract Address" }</h2>
                <a
                    id="redirectLink"
                    target="_blank"
                    href={explorer_link}
                    title="MultiversX Explorer"
                    >
                    <Icon class={classes!(redirect_visible, "redirect-icon")} icon_id={IconId::OcticonsLinkExternal16}/>
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
