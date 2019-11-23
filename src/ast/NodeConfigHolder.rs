use std::rc::Rc;
use crate::ast::convert::SqlArgTypeConvert::SqlArgTypeConvert;
use crate::engines;
use serde_json::Value;

use crate::ast::convert::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use engines::RbatisEngine::node::Node;
use crate::engines::RbatisEngine::runtime::RbatisEngine;

#[derive(Clone)]
pub struct NodeConfigHolder{
    pub sqlConvert: Rc<SqlArgTypeConvert>,
    pub engine: RbatisEngine,
}

impl NodeConfigHolder{
    pub fn new() -> Self{
        let engine= RbatisEngine::new();
        let convert=Rc::new(SqlArgTypeConvertDefault::new());

        return NodeConfigHolder{
            sqlConvert:convert,
            engine:engine,
        }
    }
}