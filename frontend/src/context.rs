use std::cell::RefCell;
use std::rc::Rc;

use crate::config::Config;
use html::ChildrenProps;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigContext {
    pub config: Rc<RefCell<Config>>,
    pub deadline: String,
    pub set_deadline: Callback<String>,
    pub set_config: Callback<Config>,
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
    let config = use_state(|| Rc::new(RefCell::new(Config::new())));
    let deadline = use_state(|| String::new());

    let set_deadline = {
        let deadline = deadline.clone();
        Callback::from(move |new_deadline: String| {
            deadline.set(new_deadline);
        })
    };

    let set_config = {
        let config = config.clone();
        Callback::from(move |new_config: Config| {
            let mut config = config.borrow_mut();
            *config = new_config;
        })
    };

    html! {
        <ContextProvider<ConfigContext> context={ConfigContext { config: (*config).clone(), deadline: (*deadline).clone(), set_deadline, set_config}}>
            { for props.children.iter() }
        </ContextProvider<ConfigContext>>
    }
}
