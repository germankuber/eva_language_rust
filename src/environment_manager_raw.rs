use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::EvalDataType;

pub struct EnvironmentManagerRaw {
    pub env: HashMap<String, EvalDataType>,
    parent: Option<Rc<RefCell<EnvironmentManagerRaw>>>,
}

impl EnvironmentManagerRaw {
    pub fn new(env: Option<HashMap<String, EvalDataType>>, parent: Option<Rc<RefCell<EnvironmentManagerRaw>>>) -> EnvironmentManagerRaw {
        EnvironmentManagerRaw {
            env: env.unwrap_or(HashMap::new()),
            parent,
        }
    }
    pub fn define(&mut self, name: String, value: EvalDataType) -> EvalDataType {
        self.env.insert(name, value.clone());
        value
    }
    pub fn assign(&mut self, name: String, value: EvalDataType) -> Option<EvalDataType> {
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
    pub fn get(&self, name: String) -> Option<EvalDataType> {
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