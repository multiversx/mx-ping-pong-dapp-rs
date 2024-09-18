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
    pub set_deadline: Callback<String>,
    pub set_config: Callback<Config>,
}

pub async fn refresh_context() -> (Config, String) {
    let config = Config::new();
    let mut deadline = String::new();
    if let Ok(new_deadline) = query::get_deadline(&config).await {
        deadline = new_deadline;
    }

    (config, deadline)
}

impl Default for ConfigContext {
    fn default() -> Self {
        ConfigContext {
            config: Rc::new(RefCell::new(Config::new())),
            deadline: String::new(),
            set_deadline: Callback::noop(),
            set_config: Callback::noop(),
        }
    }
}

#[function_component(ConfigProvider)]
pub fn config_provider(props: &ChildrenProps) -> Html {
    let config = use_state(Config::new);
    let deadline = use_state(String::new);

    let set_deadline = {
        let deadline = deadline.clone();
        Callback::from(move |new_deadline: String| {
            deadline.set(new_deadline);
        })
    };

    let set_config = {
        let config = config.clone();
        Callback::from(move |new_config: Config| {
            config.set(new_config);
        })
    };

    let set_deadline_effect = set_deadline.clone();
    let set_config_effect = set_config.clone();

    // refresh context on component mount
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let (new_config, new_deadline) = refresh_context().await;

                set_deadline_effect.emit(new_deadline);
                set_config_effect.emit(new_config);
            });
            || () // no cleanup fn
        },
        (), // empty dependency array, run once on mount
    );

    let context = ConfigContext {
        config: Rc::new(RefCell::new((*config).clone())),
        deadline: (*deadline).clone(),
        set_deadline,
        set_config,
    };

    html! {
        <ContextProvider<ConfigContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ConfigContext>>
    }
}
