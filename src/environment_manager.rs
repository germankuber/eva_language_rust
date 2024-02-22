use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::EvalType;

pub struct EnvironmentManager {
    pub env: HashMap<String, EvalType>,
    parent: Option<Rc<RefCell<EnvironmentManager>>>,
}

impl EnvironmentManager {
    pub fn new(env: Option<HashMap<String, EvalType>>, parent: Option<Rc<RefCell<EnvironmentManager>>>) -> EnvironmentManager {
        EnvironmentManager {
            env: env.unwrap_or(HashMap::new()),
            parent,
        }
    }
    pub fn define(&mut self, name: String, value: EvalType) -> EvalType {
        self.env.insert(name, value.clone());
        value
    }
    pub fn get(&self, name: String) -> Option<EvalType> {
        self.env.get(&name).map(|x| x.clone())
    }
}