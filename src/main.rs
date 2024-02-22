use std::cell::RefCell;
use std::rc::Rc;
use regex::Regex;
use crate::environment_manager::EnvironmentManager;

mod environment_manager;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug, Clone)]
enum EvalTypeOperation {
    Mul,
    Add,
}

#[derive(PartialEq, Debug, Clone)]
enum EvalTypeKeyword {
    Var,
    Set,
}

#[derive(PartialEq, Debug, Clone)]
enum EvalType {
    Number(u128),
    Boolean(bool),
    String(String),
    VariableName(String),
    Operator(EvalTypeOperation),
    Operations(Vec<EvalType>),
    Keyword(EvalTypeKeyword),
    BeginBlock(Vec<EvalType>),
}

struct Eva {}

impl Eva {
    pub fn new() -> Eva {
        Eva {}
    }
}

impl Eva {
    pub fn eval(&self, exp: Vec<EvalType>, env_manager: Rc<RefCell<EnvironmentManager>>) -> EvalType {
        match &exp[0] {
            EvalType::BeginBlock(operations) => self.evaluate_block(operations.to_vec(), env_manager),
            EvalType::Number(n) => EvalType::Number(n.clone()),
            EvalType::Boolean(n) => EvalType::Boolean(n.clone()),
            EvalType::String(n) => EvalType::String(n.clone()),
            EvalType::Operator(operation) => self.eval_operator(operation, &exp[1..].to_vec(), env_manager),
            EvalType::Operations(operations) => self.eval(operations.clone(), env_manager),
            EvalType::Keyword(keyword) => self.eval_keyword(keyword, &exp[1..].to_vec(), env_manager),
            EvalType::VariableName(var_name) => self.get_variable_value(var_name, env_manager),
        }
    }
    fn evaluate_block(&self, exp: Vec<EvalType>, env_manager: Rc<RefCell<EnvironmentManager>>) -> EvalType {
        let mut result = EvalType::Number(0);
        let block_env = Rc::new(RefCell::new(EnvironmentManager::new(None, Some(Rc::clone(&env_manager)))));

        for e in exp {
            result = self.eval(vec![e], Rc::clone(&block_env));
        }
        result
    }
    fn get_variable_value(&self, var_name: &str, env_manager: Rc<RefCell<EnvironmentManager>>) -> EvalType {
        if let Some(value) = env_manager.borrow().get(var_name.to_string()) {
            return value;
        }
        panic!("Variable {} does not exist", var_name)
    }
    fn eval_keyword(&self, keyword: &EvalTypeKeyword, exp: &Vec<EvalType>, env_manager: Rc<RefCell<EnvironmentManager>>) -> EvalType {
        match keyword {
            EvalTypeKeyword::Var => {
                if let EvalType::VariableName(var_name) = &exp[0] {
                    if !Regex::new(r"^[+\-*/<>=a-zA-Z0-9_]+$").unwrap().is_match(var_name) {
                        panic!("Invalid eval_keyword")
                    }
                    let var_value = self.get_var_value(&self.eval(exp[1..].to_vec(), Rc::clone(&env_manager)));
                    return env_manager.borrow_mut().define(var_name.clone(), var_value);
                }
            }
            EvalTypeKeyword::Set => {
                if let EvalType::VariableName(var_name) = &exp[0] {
                    let var_value = self.get_var_value(&self.eval(exp[1..].to_vec(), Rc::clone(&env_manager)));
                    if let Some(value) = env_manager.borrow_mut().assign(var_name.clone(), var_value) {
                        return value;
                    }
                    panic!("Variable {} does not exist", var_name)
                }
            }
        }

        panic!("Invalid eval_keyword")
    }
    fn get_var_value(&self, var_value: &EvalType) -> EvalType {
        match var_value {
            EvalType::Number(n) => EvalType::Number(n.clone()),
            EvalType::String(n) => EvalType::String(n.clone()),
            EvalType::Boolean(n) => EvalType::Boolean(n.clone()),
            // EvalType::Operations(operations) => self.eval(operations.clone()),
            _ => panic!("Invalid var value"),
        }
    }
    fn eval_operator(&self, exp_operation: &EvalTypeOperation, exp: &Vec<EvalType>, env_manager: Rc<RefCell<EnvironmentManager>>) -> EvalType {
        match exp_operation {
            EvalTypeOperation::Add => self.process_add(&self.eval(vec![exp[0].clone()], Rc::clone(&env_manager)), &self.eval(vec![exp[1].clone()], Rc::clone(&env_manager)), env_manager),
            EvalTypeOperation::Mul => self.process_mul(&self.eval(vec![exp[0].clone()], Rc::clone(&env_manager)), &self.eval(vec![exp[1].clone()], Rc::clone(&env_manager)), env_manager),
        }
    }
    fn process_add(&self, first: &EvalType, second: &EvalType, env_manager: Rc<RefCell<EnvironmentManager>>) -> EvalType {
        match (first, second) {
            (EvalType::Number(n1), EvalType::Number(n2)) => EvalType::Number(n1 + n2),
            (EvalType::String(s1), EvalType::String(s2)) => EvalType::String(format!("{}{}", s1, s2)),
            (EvalType::Operations(n1), EvalType::Number(n2)) => match self.eval(n1.clone(), env_manager) {
                EvalType::Number(r) => EvalType::Number(r + n2),
                _ => panic!("Invalid process_add"),
            },
            (EvalType::Number(n1), EvalType::Operations(n2)) => match self.eval(n2.clone(), env_manager) {
                EvalType::Number(r) => EvalType::Number(n1 + r),
                _ => panic!("Invalid process_add"),
            },
            (EvalType::Operations(s1), EvalType::String(s2)) => match self.eval(s1.clone(), env_manager) {
                EvalType::String(r) => EvalType::String(format!("{}{}", r, s2)),
                _ => panic!("Invalid process_add"),
            },
            (EvalType::String(s1), EvalType::Operations(s2)) => match self.eval(s2.clone(), env_manager) {
                EvalType::String(r) => EvalType::String(format!("{}{}", s1, r)),
                _ => panic!("Invalid process_add"),
            },
            _ => panic!("Invalid process_add"),
        }
    }
    fn process_mul(&self, first: &EvalType, second: &EvalType, env_manager: Rc<RefCell<EnvironmentManager>>) -> EvalType {
        match (first, second) {
            (EvalType::Number(n1), EvalType::Number(n2)) => EvalType::Number(n1 * n2),
            (EvalType::Operations(n1), EvalType::Number(n2)) => match self.eval(n1.clone(), env_manager) {
                EvalType::Number(r) => EvalType::Number(r * n2),
                _ => panic!("Invalid process_mul"),
            },
            (EvalType::Number(n1), EvalType::Operations(n2)) => match self.eval(n2.clone(), env_manager) {
                EvalType::Number(r) => EvalType::Number(n1 * r),
                _ => panic!("Invalid process_mul"),
            },
            _ => panic!("Invalid process_mul"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::vec;

    use crate::{Eva, EvalType, EvalTypeKeyword, EvalTypeOperation};
    use crate::environment_manager::EnvironmentManager;

    #[test]
    fn test_identity() {
        let eva = Eva::new();
        ;
        // Test case 1
        assert_eq!(eva.eval(vec![EvalType::Number(1)], get_environment_manager()), EvalType::Number(1));
        assert_eq!(
            eva.eval(vec![EvalType::String("Test".to_owned())], get_environment_manager()),
            EvalType::String("Test".to_owned())
        );
    }

    #[test]
    fn test_add() {
        let eva = Eva::new();
        assert_eq!(
            eva.eval(vec![
                EvalType::Operator(EvalTypeOperation::Add),
                EvalType::Number(1),
                EvalType::Number(7),
            ], get_environment_manager()),
            EvalType::Number(8)
        );
    }

    #[test]
    fn test_add_mul_complex() {
        let eva = Eva::new();
        assert_eq!(
            eva.eval(vec![EvalType::Operations(vec![
                EvalType::Operator(EvalTypeOperation::Mul),
                EvalType::Operations(vec![
                    EvalType::Operator(EvalTypeOperation::Add),
                    EvalType::Number(2),
                    EvalType::Number(2),
                ]),
                EvalType::Number(4),
            ])], get_environment_manager()),
            EvalType::Number(16)
        );
    }

    #[test]
    fn test_mul() {
        let eva = Eva::new();
        // Test case 1
        assert_eq!(
            eva.eval(vec![
                EvalType::Operator(EvalTypeOperation::Mul),
                EvalType::Number(2),
                EvalType::Number(5),
            ], get_environment_manager()),
            EvalType::Number(10)
        );
    }

    #[test]
    fn test_concat() {
        let eva = Eva::new();
        assert_eq!(
            eva.eval(vec![
                EvalType::Operator(EvalTypeOperation::Add),
                EvalType::String("Hello ".to_owned()),
                EvalType::String("world!".to_owned()),
            ], get_environment_manager()),
            EvalType::String("Hello world!".to_owned()));


        assert_eq!(
            eva.eval(vec![
                   EvalType::Operations(vec![
                       EvalType::Operator(EvalTypeOperation::Add),
                          EvalType::Operations(vec![
                       EvalType::Operator(EvalTypeOperation::Add),
                       EvalType::String("Hello ".to_owned()),
                       EvalType::String("world".to_owned()),
                   ]),
                       EvalType::String(" from Eva lang!".to_owned()),
                   ]),
            ],get_environment_manager()),
        EvalType::String("Hello world from Eva lang!".to_owned()),
        );
    }

    #[test]
    fn test_declare_variable() {
        let eva = Eva::new();
        assert_eq!(
            eva.eval(vec![
                EvalType::Keyword(EvalTypeKeyword::Var),
                EvalType::VariableName("variable_name".to_owned()),
                EvalType::Number(10),
            ],get_environment_manager()),
        EvalType::Number(10),
        );
    }

    #[test]
    fn test_get_variable() {
        let eva = Eva::new();
        let environment_manager = get_environment_manager();
        eva.eval(vec![
            EvalType::Keyword(EvalTypeKeyword::Var),
            EvalType::VariableName("variable_name".to_owned()),
            EvalType::Number(3),
        ], Rc::clone(&environment_manager));
        assert_eq!(
            eva.eval(vec![
                EvalType::VariableName("variable_name".to_owned())]
                     , Rc::clone(&environment_manager)),
            EvalType::Number(3));
    }

    #[test]
    fn test_declare_complex_variable() {
        let eva = Eva::new();
        let environment_manager = get_environment_manager();
        eva.eval(vec![
            EvalType::Operations(vec![
                EvalType::Keyword(EvalTypeKeyword::Var),
                EvalType::VariableName("variable_name".to_owned()),
                EvalType::Operations(vec![
                    EvalType::Operator(EvalTypeOperation::Add),
                    EvalType::String("Hello ".to_owned()),
                    EvalType::String("world!".to_owned()),
                ]),
            ]),
        ], Rc::clone(&environment_manager));

        assert_eq!(
            eva.eval(vec![
                EvalType::VariableName("variable_name".to_owned())]
                     , Rc::clone(&environment_manager)),
            EvalType::String("Hello world!".to_owned()));
    }

    #[test]
    fn test_get_variable_bool() {
        let eva = Eva::new();
        let environment_manager = get_environment_manager();
        eva.eval(vec![
            EvalType::Keyword(EvalTypeKeyword::Var),
            EvalType::VariableName("variable_name".to_owned()),
            EvalType::Boolean(true),
        ], Rc::clone(&environment_manager));
        assert_eq!(
            eva.eval(vec![
                EvalType::VariableName("variable_name".to_owned())]
                     , Rc::clone(&environment_manager)),
            EvalType::Boolean(true));
    }

    #[test]
    fn test_begin_block() {
        assert_eq!(
            Eva::new().eval(vec![
                EvalType::BeginBlock(vec![
                    EvalType::Operations(vec![
                        EvalType::Keyword(EvalTypeKeyword::Var),
                        EvalType::VariableName("variable_a".to_owned()),
                        EvalType::Number(10),
                    ]),
                    EvalType::Operations(vec![
                        EvalType::Keyword(EvalTypeKeyword::Var),
                        EvalType::VariableName("variable_b".to_owned()),
                        EvalType::Number(20),
                    ]),
                    EvalType::Operations(vec![
                        EvalType::Operator(EvalTypeOperation::Add),
                        EvalType::Operations(vec![
                            EvalType::Operator(EvalTypeOperation::Mul),
                            EvalType::VariableName("variable_a".to_owned()),
                            EvalType::VariableName("variable_b".to_owned()),
                        ]),
                        EvalType::Number(30),
                    ]),
                ]),
            ], get_environment_manager()),
            EvalType::Number(230));
    }

    #[test]
    fn test_begin_block_concatenate() {
        assert_eq!(
            Eva::new().eval(vec![
                EvalType::BeginBlock(vec![
                    EvalType::Operations(vec![
                        EvalType::Keyword(EvalTypeKeyword::Var),
                        EvalType::VariableName("variable_a".to_owned()),
                        EvalType::Number(10),
                    ]),
                    EvalType::BeginBlock(vec![
                        EvalType::Operations(vec![
                            EvalType::Keyword(EvalTypeKeyword::Var),
                            EvalType::VariableName("variable_a".to_owned()),
                            EvalType::Number(20),
                        ]),
                        EvalType::VariableName("variable_a".to_owned()),
                    ]),
                    EvalType::VariableName("variable_a".to_owned()),
                ]),
            ], get_environment_manager()),
            EvalType::Number(10));
    }

    #[test]
    fn test_begin_block_concatenate_assign_variable() {
        assert_eq!(
            Eva::new().eval(vec![
                EvalType::BeginBlock(vec![
                    EvalType::Operations(vec![
                        EvalType::Keyword(EvalTypeKeyword::Var),
                        EvalType::VariableName("value".to_owned()),
                        EvalType::Number(10),
                    ]),
                    EvalType::Operations(vec![
                        EvalType::Keyword(EvalTypeKeyword::Var),
                        EvalType::VariableName("result".to_owned()),
                        EvalType::BeginBlock(vec![
                            EvalType::Operations(vec![
                                EvalType::Keyword(EvalTypeKeyword::Var),
                                EvalType::VariableName("x".to_owned()),
                                EvalType::Operations(vec![
                                    EvalType::Operator(EvalTypeOperation::Add),
                                    EvalType::VariableName("value".to_owned()),
                                    EvalType::Number(10),
                                ]),
                                EvalType::VariableName("x".to_owned()),
                            ]),
                        ]),
                    ]),
                    EvalType::VariableName("result".to_owned()),
                ]),
            ], get_environment_manager()),
            EvalType::Number(20));
    }

    #[test]
    fn test_assign_variable() {
        assert_eq!(
            Eva::new().eval(vec![
                EvalType::BeginBlock(vec![
                    EvalType::Operations(vec![
                        EvalType::Keyword(EvalTypeKeyword::Var),
                        EvalType::VariableName("data".to_owned()),
                        EvalType::Number(10),
                    ]),
                    EvalType::BeginBlock(vec![
                        EvalType::Operations(vec![
                            EvalType::Keyword(EvalTypeKeyword::Set),
                            EvalType::VariableName("data".to_owned()),
                            EvalType::Number(100),
                        ]),
                    ]),
                    EvalType::VariableName("data".to_owned()),
                ]),
            ], get_environment_manager()),
            EvalType::Number(100));
    }

    #[test]
    fn get_global_variable() {
        let eva = Eva::new();
        let environment_manager = get_environment_manager();

        assert_eq!(
            eva.eval(vec![
                EvalType::VariableName("VERSION".to_owned())]
                     , Rc::clone(&environment_manager)),
            EvalType::String("1.0.0".to_owned()));
    }

    fn get_environment_manager() -> Rc<RefCell<EnvironmentManager>> {
        Rc::new(RefCell::new(EnvironmentManager::new(Some(HashMap::from([
            ("VERSION".to_owned(), EvalType::String("1.0.0".to_owned())),
        ])), None)))
    }
}
