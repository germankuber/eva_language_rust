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
    pub fn assign(&mut self, name: String, value: EvalType) -> Option<EvalType> {
        if self.env.contains_key(&name) {
            self.env.insert(name, value.clone());
            return Some(value);
        } else {
            if let Some(parent) = &self.parent {
                return parent.borrow_mut().assign(name, value);
            }
        }
        None
    }
    pub fn get(&self, name: String) -> Option<EvalType> {
        if let Some(value) = self.env.get(&name) {
            return Some(value.clone());
        } else {
            if let Some(parent) = &self.parent {
                return parent.borrow().get(name);
            }
        }
        None
    }
}