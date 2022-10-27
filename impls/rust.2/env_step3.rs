use crate::maltypes_step3::{MalType};
use crate::printer_step3::pr_str;
use std::collections::{HashMap, VecDeque};

pub struct MalNameSpace(HashMap<String, MalType>);
pub struct MalEnv(VecDeque<MalNameSpace>);

impl MalEnv {
    pub fn set(&mut self, new_sym: &str, val: MalType) {
        match self.0.front_mut() {
            Some(ns) => {
                ns.0.insert(new_sym.to_string(), val);
            }
            None => {}
        }
    }

    pub fn get(&self, sym: &String) -> Option<MalType> {
        for env in self.0.iter() {
            if let Some(val) = env.0.get(sym) {
                return Some(val.clone())
            }
        }
        None
    }

    pub fn new_env(&mut self, binds_opt: Option<Vec<(String, MalType)>>) {
        let env = HashMap::<String, MalType>::new();
        self.0.push_front(MalNameSpace(env));
        if let Some(binds) = binds_opt {
            for pair in binds.iter() {
                self.set(&pair.0, pair.1.clone());
            }
        }
    }

    pub fn drop_env(&mut self) {
        self.0.pop_front().unwrap();
    }

    pub fn new() -> MalEnv {
        let global_env = HashMap::<String, MalType>::new();
        let mut new_q = VecDeque::new();
        new_q.push_front(MalNameSpace(global_env));
        return MalEnv(new_q);
    }

    pub fn _prt(&self) -> String {
        let mut outline = String::new();
        for (env_count, env) in self.0.iter().enumerate() {
            outline.push_str(format!("Environment {}:\n", env_count).as_str());
            for (sym_name, maltype) in env.0.iter() {
                outline.push_str(format!("sym: {} => ", sym_name).as_str());
                outline.push_str(pr_str(maltype, true).as_str());
                outline.push_str("\n")
            }
        }
        return outline
    }
}

