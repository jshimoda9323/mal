use crate::maltypes::{MalType, mal_add, mal_sub, mal_mul, mal_div};
use crate::printer::pr_str;
use std::collections::{HashMap, VecDeque};

pub struct MalEnv(VecDeque<HashMap<String, MalType>>);

impl MalEnv {
    pub fn set(&mut self, new_sym: &String, val: MalType) {
        match self.0.front_mut() {
            Some(env) => {
                env.insert(new_sym.clone(), val);
            }
            None => {
                let mut env = HashMap::<String, MalType>::new();
                env.insert(new_sym.clone(), val);
                self.0.push_front(env);
            }
        }
    }

    pub fn get(&self, sym: &String) -> Option<MalType> {
        for env in self.0.iter() {
            match env.get(sym) {
                Some(val) => return Some(val.clone()),
                None => {}
            }
        }
        None
    }

    pub fn new_env(&mut self) {
        let env = HashMap::<String, MalType>::new();
        self.0.push_front(env);
    }

    pub fn drop_env(&mut self) {
        self.0.pop_front().unwrap();
    }

    pub fn new() -> MalEnv {
        let mut global_env = HashMap::new();
        global_env.insert("+".to_string(),MalType::Operator(mal_add));
        global_env.insert("-".to_string(),MalType::Operator(mal_sub));
        global_env.insert("*".to_string(),MalType::Operator(mal_mul));
        global_env.insert("/".to_string(),MalType::Operator(mal_div));
        let mut new_q = VecDeque::new();
        new_q.push_front(global_env);
        return MalEnv(new_q);
    }

    pub fn prt(&self) -> String {
        let mut outline = String::new();
        for (env_count, env) in self.0.iter().enumerate() {
            outline.push_str(format!("Environment {}:\n", env_count).as_str());
            for (sym_name, maltype) in env.iter() {
                outline.push_str(format!("sym: {} => ", sym_name).as_str());
                outline.push_str(pr_str(maltype, true).as_str());
                outline.push_str("\n")
            }
        }
        return outline
    }
}

