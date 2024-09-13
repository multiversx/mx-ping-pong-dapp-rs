use std::cell::RefCell;
use std::rc::Rc;

use crate::config::Config;
use html::ChildrenProps;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigContext {
    pub config: Rc<RefCell<Config>>,
    pub deadline: String,
    pub timestamp: String,
    pub max_funds: String,
    pub ping_amount: String,
    pub contract_address: String,
    pub set_deadline: Callback<String>,
    pub set_timestamp: Callback<String>,
    pub set_max_funds: Callback<String>,
    pub set_ping_amount: Callback<String>,
    pub set_contract_address: Callback<String>,
    pub set_config: Callback<Config>,
}

impl Default for ConfigContext {
    fn default() -> Self {
        ConfigContext {
            config: Rc::new(RefCell::new(Config::new())),
            deadline: String::new(),
            timestamp: String::new(),
            max_funds: String::new(),
            ping_amount: String::new(),
            contract_address: String::new(),
            set_deadline: Callback::noop(),
            set_timestamp: Callback::noop(),
            set_max_funds: Callback::noop(),
            set_ping_amount: Callback::noop(),
            set_contract_address: Callback::noop(),
            set_config: Callback::noop(),
        }
    }
}

#[function_component(ConfigProvider)]
pub fn config_provider(props: &ChildrenProps) -> Html {
    let config = use_state(|| Rc::new(RefCell::new(Config::new())));
    let deadline: UseStateHandle<String> = use_state(String::new);
    let timestamp: UseStateHandle<String> = use_state(String::new);
    let max_funds: UseStateHandle<String> = use_state(String::new);
    let ping_amount: UseStateHandle<String> = use_state(String::new);
    let contract_address = use_state(String::new);

    let set_deadline = {
        let deadline = deadline.clone();
        Callback::from(move |new_deadline: String| {
            deadline.set(new_deadline);
        })
    };

    let set_timestamp = {
        let timestamp = timestamp.clone();
        Callback::from(move |new_timestamp: String| {
            timestamp.set(new_timestamp);
        })
    };

    let set_max_funds = {
        let max_funds = max_funds.clone();
        Callback::from(move |new_max_funds: String| {
            max_funds.set(new_max_funds);
        })
    };

    let set_ping_amount = {
        let ping_amount = ping_amount.clone();
        Callback::from(move |new_ping_amount: String| {
            ping_amount.set(new_ping_amount);
        })
    };

    let set_config = {
        let config = config.clone();
        Callback::from(move |new_config: Config| {
            let mut config = config.borrow_mut();
            *config = new_config;
        })
    };

    let set_contract_address = {
        let contract_address = contract_address.clone();
        Callback::from(move |new_contract_address: String| {
            contract_address.set(new_contract_address);
        })
    };

    html! {
        <ContextProvider<ConfigContext> context = {
            ConfigContext {
            config: (*config).clone(),
            deadline: (*deadline).clone(),
            timestamp: (*timestamp).clone(),
            max_funds: (*max_funds).clone(),
            ping_amount: (*ping_amount).clone(),
            contract_address: (*contract_address).clone(),
            set_deadline,
            set_timestamp,
            set_max_funds,
            set_ping_amount,
            set_contract_address,
            set_config,
            }}>
            { for props.children.iter() }
        </ContextProvider<ConfigContext>>
    }
}
