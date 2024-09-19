use crate::config::Config;
use crate::requests::{query, ContractState};
use html::ChildrenProps;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigContext {
    pub config: Rc<RefCell<Config>>,
    pub contract_state: ContractState,
    pub set_contract_state: Callback<ContractState>,
    pub set_config: Callback<Config>,
}

pub async fn refresh_context() -> (Config, ContractState) {
    log::info!("refreshing context");
    let config = Config::new();

    let contract_state = query::get_contract_state(&config).await.unwrap_or_default();

    (config, contract_state)
}

impl Default for ConfigContext {
    fn default() -> Self {
        ConfigContext {
            config: Rc::new(RefCell::new(Config::new())),
            contract_state: ContractState::default(),
            set_contract_state: Callback::noop(),
            set_config: Callback::noop(),
        }
    }
}

#[function_component(ConfigProvider)]
pub fn config_provider(props: &ChildrenProps) -> Html {
    let config = use_state(Config::new);
    let contract_state = use_state(ContractState::default);

    let set_config = {
        let config = config.clone();
        Callback::from(move |new_config: Config| {
            config.set(new_config);
        })
    };

    let set_contract_state = {
        let contract_state = contract_state.clone();
        Callback::from(move |new_contract_state: ContractState| {
            contract_state.set(new_contract_state);
        })
    };

    let set_config_effect = set_config.clone();
    let set_contract_state_effect = set_contract_state.clone();

    // refresh context on component mount
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let (new_config, new_contract_state) = refresh_context().await;
                set_config_effect.emit(new_config);
                set_contract_state_effect.emit(new_contract_state);
            });
            || () // no cleanup fn
        },
        (), // empty dependency array, run once on mount
    );

    let context = ConfigContext {
        config: Rc::new(RefCell::new((*config).clone())),
        contract_state: (*contract_state).clone(),
        set_contract_state,
        set_config,
    };

    html! {
        <ContextProvider<ConfigContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ConfigContext>>
    }
}
