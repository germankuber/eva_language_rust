extern crate core;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use regex::{Regex};
use crate::environment_manager_raw::EnvironmentManagerRaw;


mod environment_manager_raw;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug, Clone)]
enum EvalType {
    Content(Vec<EvalType>),
    Value(EvalDataType),
}

#[derive(PartialEq, Debug, Clone)]
pub enum EvalDataType {
    String(String),
    Number(u128),
    Bool(bool),
}

struct Eva {}

impl Eva {
    pub fn new() -> Eva {
        Eva {}
    }
}

impl Eva {
    pub fn eval(&self, exp: Vec<EvalType>, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        match &exp[0] {
            EvalType::Content(v) => self.eval(v.clone(), env_manager),
            EvalType::Value(v) => self.evaluate_eval_data(v, &exp[1..].to_vec(), env_manager)
        }
    }
    pub fn evaluate_eval_data(&self, eval_data_type: &EvalDataType, exp: &Vec<EvalType>, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        match eval_data_type {
            EvalDataType::String(v) => self.process_operation(v, exp, env_manager),
            EvalDataType::Number(v) => EvalType::Value(EvalDataType::Number(v.clone())),
            EvalDataType::Bool(v) => EvalType::Value(EvalDataType::Bool(v.clone())),
        }
    }
    pub fn process_operation(&self, operation: &str, exp: &Vec<EvalType>, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        match operation {
            "+" => self.process_add(&self.eval(exp[0..1].to_vec(), Rc::clone(&env_manager)), &self.eval(exp[1..].to_vec(), Rc::clone(&env_manager)), Rc::clone(&env_manager)),
            ">" => self.process_bigger(&self.eval(exp[0..1].to_vec(), Rc::clone(&env_manager)), &self.eval(exp[1..].to_vec(), Rc::clone(&env_manager)), Rc::clone(&env_manager)),
            "<" => self.process_smaller(&self.eval(exp[0..1].to_vec(), Rc::clone(&env_manager)), &self.eval(exp[1..].to_vec(), Rc::clone(&env_manager)), Rc::clone(&env_manager)),
            "*" => self.process_mul(&self.eval(exp[0..1].to_vec(), Rc::clone(&env_manager)), &self.eval(exp[1..].to_vec(), Rc::clone(&env_manager)), Rc::clone(&env_manager)),
            "/" => self.process_div(&self.eval(exp[0..1].to_vec(), Rc::clone(&env_manager)), &self.eval(exp[1..].to_vec(), Rc::clone(&env_manager)), Rc::clone(&env_manager)),
            "var" => self.process_variable_declaration(&exp[0], &self.eval(exp[1..].to_vec(), Rc::clone(&env_manager)), Rc::clone(&env_manager)),
            "set" => self.process_set_variable(&exp[0], &self.eval(exp[1..].to_vec(), Rc::clone(&env_manager)), Rc::clone(&env_manager)),
            "if" => self.process_if_else(&exp, Rc::clone(&env_manager)),
            "while" => self.process_while(&exp, Rc::clone(&env_manager)),
            "begin" => self.process_begin(exp[0].clone(), Rc::clone(&env_manager)),
            v => self.process_value_string(v, Rc::clone(&env_manager)),
        }
    }
    pub fn process_bigger(&self, first: &EvalType, second: &EvalType, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        match (first, second) {
            (EvalType::Value(EvalDataType::Number(a)), EvalType::Value(EvalDataType::Number(b))) => EvalType::Value(EvalDataType::Bool(a > b)),
            _ => panic!("process_add does not supported types: {:?}, {:?}", first, second)
        }
    }
    pub fn process_smaller(&self, first: &EvalType, second: &EvalType, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        match (first, second) {
            (EvalType::Value(EvalDataType::Number(a)), EvalType::Value(EvalDataType::Number(b))) => EvalType::Value(EvalDataType::Bool(a < b)),
            _ => panic!("process_add does not supported types: {:?}, {:?}", first, second)
        }
    }
    pub fn process_value_string(&self, value: &str, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        match Regex::new(r"^'(?P<value>.*)'$").unwrap().captures(value) {
            Some(captures) => EvalType::Value(EvalDataType::String(captures.name("value").unwrap().as_str().to_owned())),
            None => self.process_get_variable(value, Rc::clone(&env_manager))
        }
    }
    pub fn process_add(&self, first: &EvalType, second: &EvalType, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        match (first, second) {
            (EvalType::Value(EvalDataType::Number(a)), EvalType::Value(EvalDataType::Number(b))) => EvalType::Value(EvalDataType::Number(a + b)),
            (EvalType::Value(EvalDataType::String(a)), EvalType::Value(EvalDataType::String((b)))) => EvalType::Value(EvalDataType::String(format!("{}{}", a, b))),
            _ => panic!("process_add does not supported types: {:?}, {:?}", first, second)
        }
    }
    pub fn process_if_else(&self, exp: &Vec<EvalType>, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        if let EvalType::Content(exp_content) = &exp[0] {
            match self.eval(vec![exp_content[0].clone()], Rc::clone(&env_manager)) {
                EvalType::Value(EvalDataType::Bool(value)) => {
                    if value {
                        return self.eval(vec![exp_content[1].clone()], Rc::clone(&env_manager));
                    }
                    return self.eval(vec![exp_content[2].clone()], Rc::clone(&env_manager));
                }
                _ => panic!("process_if_else does not supported types: {:?}", exp_content[0])
            }
        }
        panic!("process_if_else does not supported types: {:?}", exp[0])
    }
    pub fn process_while(&self, exp: &Vec<EvalType>, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        if let EvalType::Content(exp_content) = &exp[0] {
            let mut result = EvalType::Value(EvalDataType::Number(0));
            while true {
                match self.eval(vec![exp_content[0].clone()], Rc::clone(&env_manager)) {
                    EvalType::Value(EvalDataType::Bool(value)) => {
                        if !value{
                            return result;
                        }
                        result = self.eval(vec![exp_content[1].clone()], Rc::clone(&env_manager));
                    }
                    _ => panic!("process_if_else does not supported types: {:?}", exp_content[0])
                }
            }
        }
        panic!("process_if_else does not supported types: {:?}", exp[0])
    }
    pub fn process_mul(&self, first: &EvalType, second: &EvalType, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        match (first, second) {
            (EvalType::Value(EvalDataType::Number(a)), EvalType::Value(EvalDataType::Number(b))) => EvalType::Value(EvalDataType::Number(a * b)),
            _ => panic!("process_mul does not supported types: {:?}, {:?}", first, second)
        }
    }
    pub fn process_div(&self, first: &EvalType, second: &EvalType, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        match (first, second) {
            (EvalType::Value(EvalDataType::Number(a)), EvalType::Value(EvalDataType::Number(b))) => EvalType::Value(EvalDataType::Number(a / b)),
            _ => panic!("process_mul does not supported types: {:?}, {:?}", first, second)
        }
    }
    pub fn process_variable_declaration(&self, first: &EvalType, second: &EvalType, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        match (first, second) {
            (EvalType::Value(EvalDataType::String(variable_name)), EvalType::Value(EvalDataType::Number(variable_value))) => EvalType::Value(env_manager.borrow_mut().define(variable_name.to_owned(), EvalDataType::Number(variable_value.clone()))),
            (EvalType::Value(EvalDataType::String(variable_name)), EvalType::Value(EvalDataType::String(variable_value))) => EvalType::Value(env_manager.borrow_mut().define(variable_name.to_owned(), EvalDataType::String(variable_value.clone()))),
            _ => panic!("process_mul does not supported types: {:?}, {:?}", first, second)
        }
    }
    pub fn process_set_variable(&self, first: &EvalType, second: &EvalType, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        if let Some(value) = match (first, second) {
            (EvalType::Value(EvalDataType::String(variable_name)), EvalType::Value(EvalDataType::Number(variable_value))) => env_manager.borrow_mut().assign(variable_name.to_owned(), EvalDataType::Number(variable_value.clone())),
            (EvalType::Value(EvalDataType::String(variable_name)), EvalType::Value(EvalDataType::String(variable_value))) => env_manager.borrow_mut().assign(variable_name.to_owned(), EvalDataType::String(variable_value.clone())),
            _ => panic!("process_set_variable does not supported types: {:?}, {:?}", first, second)
        } {
            return EvalType::Value(value);
        }
        panic!("process_set_variable does not supported types: {:?}, {:?}", first, second)
    }
    pub fn process_get_variable(&self, var_name: &str, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        if let Some(value) = env_manager.borrow().get(var_name.to_string()) {
            return EvalType::Value(value);
        }
        panic!("Variable {} does not exist", var_name)
    }

    fn process_begin(&self, exp: EvalType, env_manager: Rc<RefCell<EnvironmentManagerRaw>>) -> EvalType {
        let mut result = EvalType::Value(EvalDataType::Number(0));
        let block_env = Rc::new(RefCell::new(EnvironmentManagerRaw::new(None, Some(Rc::clone(&env_manager)))));

        if let EvalType::Content(exp) = exp {
            for e in exp {
                result = self.eval(vec![e], Rc::clone(&block_env));
            }
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::{Eva, EvalDataType, EvalType, get_environment_manager};

    #[test]
    fn test_identity() {
        let eva = Eva::new();
        assert_eq!(eva.eval(vec![EvalType::Value(EvalDataType::Number(1))], get_environment_manager()), EvalType::Value(EvalDataType::Number(1)));
        assert_eq!(eva.eval(vec![EvalType::Value(EvalDataType::String("'data to check'".to_owned()))], get_environment_manager()), EvalType::Value(EvalDataType::String("data to check".to_owned())));
    }

    #[test]
    fn test_add() {
        let eva = Eva::new();
        assert_eq!(eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("+".to_owned())),
                EvalType::Value(EvalDataType::Number(1)),
                EvalType::Value(EvalDataType::Number(3)),
            ])
        ], get_environment_manager()), EvalType::Value(EvalDataType::Number(4)));
    }

    #[test]
    fn test_mul() {
        let eva = Eva::new();
        assert_eq!(eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("*".to_owned())),
                EvalType::Value(EvalDataType::Number(2)),
                EvalType::Value(EvalDataType::Number(3)),
            ])
        ], get_environment_manager()), EvalType::Value(EvalDataType::Number(6)));
    }

    #[test]
    fn test_div() {
        let eva = Eva::new();
        assert_eq!(eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("/".to_owned())),
                EvalType::Value(EvalDataType::Number(10)),
                EvalType::Value(EvalDataType::Number(2)),
            ])
        ], get_environment_manager()), EvalType::Value(EvalDataType::Number(5)));
    }

    #[test]
    fn test_declare_variable() {
        let eva = Eva::new();
        assert_eq!(eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("var".to_owned())),
                EvalType::Value(EvalDataType::String("x".to_owned())),
                EvalType::Value(EvalDataType::Number(8)),
            ])
        ], get_environment_manager()), EvalType::Value(EvalDataType::Number(8)));

        assert_eq!(eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("var".to_owned())),
                EvalType::Value(EvalDataType::String("x".to_owned())),
                EvalType::Value(EvalDataType::String("'value'".to_owned())),
            ])
        ], get_environment_manager()), EvalType::Value(EvalDataType::String("value".to_owned())));
        assert_eq!(eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("var".to_owned())),
                EvalType::Value(EvalDataType::String("x".to_owned())),
                EvalType::Value(EvalDataType::Number(88)),
            ])
        ], get_environment_manager()), EvalType::Value(EvalDataType::Number(88)));
    }

    #[test]
    fn test_block() {
        let eva = Eva::new();
        assert_eq!(eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("begin".to_owned())),
                EvalType::Content(vec![
                    EvalType::Value(EvalDataType::String("var".to_owned())),
                    EvalType::Value(EvalDataType::String("x".to_owned())),
                    EvalType::Value(EvalDataType::Number(10)),
                ]),
                EvalType::Content(vec![
                    EvalType::Value(EvalDataType::String("var".to_owned())),
                    EvalType::Value(EvalDataType::String("y".to_owned())),
                    EvalType::Value(EvalDataType::Number(10)),
                ]),
                EvalType::Content(vec![
                    EvalType::Value(EvalDataType::String("+".to_owned())),
                    EvalType::Content(vec![
                        EvalType::Value(EvalDataType::String("*".to_owned())),
                        EvalType::Value(EvalDataType::String("x".to_owned())),
                        EvalType::Value(EvalDataType::String("y".to_owned())),
                    ]),
                    EvalType::Value(EvalDataType::Number(10)),
                ]),
            ])
        ], get_environment_manager()), EvalType::Value(EvalDataType::Number(110)));
    }

    #[test]
    fn test_nested_block() {
        let eva = Eva::new();
        assert_eq!(eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("begin".to_owned())),
                EvalType::Content(vec![
                    EvalType::Value(EvalDataType::String("var".to_owned())),
                    EvalType::Value(EvalDataType::String("x".to_owned())),
                    EvalType::Value(EvalDataType::Number(10)),
                    EvalType::Value(EvalDataType::String("begin".to_owned())),
                    EvalType::Content(vec![
                        EvalType::Value(EvalDataType::String("var".to_owned())),
                        EvalType::Value(EvalDataType::String("x".to_owned())),
                        EvalType::Value(EvalDataType::Number(20)),
                        EvalType::Value(EvalDataType::String("x".to_owned())),
                    ]),
                ]),
                EvalType::Value(EvalDataType::String("x".to_owned())),
            ])
        ], get_environment_manager()), EvalType::Value(EvalDataType::Number(10)));
    }

    #[test]
    fn test_nested_block_variable() {
        let eva = Eva::new();
        assert_eq!(eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("begin".to_owned())),
                EvalType::Content(vec![
                    EvalType::Content(vec![
                        EvalType::Value(EvalDataType::String("var".to_owned())),
                        EvalType::Value(EvalDataType::String("value".to_owned())),
                        EvalType::Value(EvalDataType::Number(10)),
                    ]),
                    EvalType::Content(vec![
                        EvalType::Value(EvalDataType::String("var".to_owned())),
                        EvalType::Value(EvalDataType::String("result".to_owned())),
                        EvalType::Value(EvalDataType::String("begin".to_owned())),
                        EvalType::Content(vec![
                            EvalType::Content(vec![
                                EvalType::Value(EvalDataType::String("var".to_owned())),
                                EvalType::Value(EvalDataType::String("x".to_owned())),
                                EvalType::Content(vec![
                                    EvalType::Value(EvalDataType::String("+".to_owned())),
                                    EvalType::Value(EvalDataType::String("value".to_owned())),
                                    EvalType::Value(EvalDataType::Number(10)),
                                ]),
                                EvalType::Value(EvalDataType::String("x".to_owned())),
                            ]),
                        ]),
                        EvalType::Value(EvalDataType::String("result".to_owned())),
                    ]),
                ]),
            ])
        ], get_environment_manager()), EvalType::Value(EvalDataType::Number(20)));
    }

    #[test]
    fn test_set_variable() {
        let eva = Eva::new();
        let env_manager = get_environment_manager();
        eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("var".to_owned())),
                EvalType::Value(EvalDataType::String("x".to_owned())),
                EvalType::Value(EvalDataType::Number(8)),
            ])
        ], Rc::clone(&env_manager));

        assert_eq!(eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("set".to_owned())),
                EvalType::Value(EvalDataType::String("x".to_owned())),
                EvalType::Value(EvalDataType::Number(11)),
            ])
        ], Rc::clone(&env_manager)), EvalType::Value(EvalDataType::Number(11)));
    }

    #[test]
    fn test_if_else() {
        let eva = Eva::new();
        assert_eq!(eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("begin".to_owned())),
                EvalType::Content(vec![
                    EvalType::Content(vec![
                        EvalType::Value(EvalDataType::String("var".to_owned())),
                        EvalType::Value(EvalDataType::String("x".to_owned())),
                        EvalType::Value(EvalDataType::Number(10)),
                    ]),
                    EvalType::Content(vec![
                        EvalType::Value(EvalDataType::String("var".to_owned())),
                        EvalType::Value(EvalDataType::String("y".to_owned())),
                        EvalType::Value(EvalDataType::Number(0)),
                    ]),
                    EvalType::Content(vec![
                        EvalType::Value(EvalDataType::String("if".to_owned())),
                        EvalType::Content(vec![
                            EvalType::Content(vec![
                                EvalType::Value(EvalDataType::String(">".to_owned())),
                                EvalType::Value(EvalDataType::String("x".to_owned())),
                                EvalType::Value(EvalDataType::Number(10)),
                            ]),
                            EvalType::Content(vec![
                                EvalType::Value(EvalDataType::String("set".to_owned())),
                                EvalType::Value(EvalDataType::String("y".to_owned())),
                                EvalType::Value(EvalDataType::Number(20)),
                            ]),
                            EvalType::Content(vec![
                                EvalType::Value(EvalDataType::String("set".to_owned())),
                                EvalType::Value(EvalDataType::String("y".to_owned())),
                                EvalType::Value(EvalDataType::Number(30)),
                            ]),
                        ]),
                    ]),
                ]),
                EvalType::Value(EvalDataType::String("x".to_owned())),
            ])
        ], get_environment_manager()), EvalType::Value(EvalDataType::Number(30)));
    }

    #[test]
    fn test_while() {
        let eva = Eva::new();
        assert_eq!(eva.eval(vec![
            EvalType::Content(vec![
                EvalType::Value(EvalDataType::String("begin".to_owned())),
                EvalType::Content(vec![
                    EvalType::Content(vec![
                        EvalType::Value(EvalDataType::String("var".to_owned())),
                        EvalType::Value(EvalDataType::String("counter".to_owned())),
                        EvalType::Value(EvalDataType::Number(0)),
                    ]),
                    EvalType::Content(vec![
                        EvalType::Value(EvalDataType::String("var".to_owned())),
                        EvalType::Value(EvalDataType::String("result".to_owned())),
                        EvalType::Value(EvalDataType::Number(0)),
                    ]),
                    EvalType::Content(vec![
                        EvalType::Value(EvalDataType::String("while".to_owned())),
                        EvalType::Content(vec![
                            EvalType::Content(vec![
                                EvalType::Value(EvalDataType::String("<".to_owned())),
                                EvalType::Value(EvalDataType::String("counter".to_owned())),
                                EvalType::Value(EvalDataType::Number(10)),
                            ]),
                            EvalType::Content(vec![
                                EvalType::Value(EvalDataType::String("begin".to_owned())),
                                EvalType::Content(vec![
                                    EvalType::Content(vec![
                                        EvalType::Value(EvalDataType::String("set".to_owned())),
                                        EvalType::Value(EvalDataType::String("result".to_owned())),
                                        EvalType::Content(vec![
                                            EvalType::Value(EvalDataType::String("+".to_owned())),
                                            EvalType::Value(EvalDataType::String("result".to_owned())),
                                            EvalType::Value(EvalDataType::Number(1)),
                                        ]),
                                    ]),
                                    EvalType::Content(vec![
                                        EvalType::Value(EvalDataType::String("set".to_owned())),
                                        EvalType::Value(EvalDataType::String("counter".to_owned())),
                                        EvalType::Content(vec![
                                            EvalType::Value(EvalDataType::String("+".to_owned())),
                                            EvalType::Value(EvalDataType::String("counter".to_owned())),
                                            EvalType::Value(EvalDataType::Number(1)),
                                        ]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        EvalType::Value(EvalDataType::String("result".to_owned())),
                    ]),
                ]),
                EvalType::Value(EvalDataType::String("x".to_owned())),
            ])
        ], get_environment_manager()), EvalType::Value(EvalDataType::Number(10)));
    }
}


fn get_environment_manager() -> Rc<RefCell<EnvironmentManagerRaw>> {
    Rc::new(RefCell::new(EnvironmentManagerRaw::new(Some(HashMap::from([
        ("VERSION".to_owned(), EvalDataType::String("1.0.0".to_owned())),
    ])), None)))
}