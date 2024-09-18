use crate::config::Config;
use crate::requests::query;
use html::ChildrenProps;
use std::cell::RefCell;
use std::rc::Rc;
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

pub async fn refresh_context() -> (Config, String, String, String, String, String) {
    let config = Config::new();
    let mut deadline = String::new();
    if let Ok(new_deadline) = query::get_deadline(&config).await {
        deadline = new_deadline["response"].as_str().unwrap().to_string();
    }

    let mut timestamp = String::new();
    if let Ok(new_timestamp) = query::get_timestamp(&config).await {
        timestamp = new_timestamp["response"].as_str().unwrap().to_string();
    }

    let mut max_funds = String::new();
    if let Ok(new_max_funds) = query::get_max_funds(&config).await {
        max_funds = new_max_funds["response"].as_str().unwrap().to_string();
    }

    let mut ping_amount = String::new();
    if let Ok(new_ping_amount) = query::get_ping_amount(&config).await {
        ping_amount = new_ping_amount["response"].as_str().unwrap().to_string();
    }

    let mut contract_address = String::new();
    if let Ok(new_contract_address) = query::get_contract_addr(&config).await {
        contract_address = new_contract_address["contract_address"]
            .as_str()
            .unwrap()
            .to_string();
    }

    (
        config,
        deadline,
        timestamp,
        max_funds,
        ping_amount,
        contract_address,
    )
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
            config.set(Rc::new(RefCell::new(new_config)));
        })
    };

    let set_contract_address = {
        let contract_address = contract_address.clone();
        Callback::from(move |new_contract_address: String| {
            contract_address.set(new_contract_address);
        })
    };

    let set_deadline_effect = set_deadline.clone();
    let set_config_effect = set_config.clone();
    let set_timestamp_effect = set_timestamp.clone();
    let set_max_funds_effect = set_max_funds.clone();
    let set_ping_amount_effect = set_ping_amount.clone();
    let set_contract_address_effect = set_contract_address.clone();

    // refresh context on component mount
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let (new_config, new_deadline, timestamp, max_funds, ping_amount, contract_address) =
                    refresh_context().await;

                set_deadline_effect.emit(new_deadline);
                set_config_effect.emit(new_config);
                set_timestamp_effect.emit(timestamp);
                set_max_funds_effect.emit(max_funds);
                set_ping_amount_effect.emit(ping_amount);
                set_contract_address_effect.emit(contract_address);
            });
            || () // no cleanup fn
        },
        (), // empty dependency array, run once on mount
    );

    let context = ConfigContext {
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
    };

    html! {
        <ContextProvider<ConfigContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ConfigContext>>
    }
}
