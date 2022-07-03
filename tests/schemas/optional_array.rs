
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Variable {
    pub oid: Option<Vec<i64>>,
    pub val: Option<String>,
}

fn default_variable_oid() -> Option<Vec<i64>> { None }

fn default_variable_val() -> Option<String> { None }

impl Default for Variable {
    fn default() -> Variable {
        Variable {
            oid: default_variable_oid(),
            val: default_variable_val(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TrapV1 {
    pub var: Option<Vec<Variable>>,
}

fn default_trapv1_var() -> Option<Vec<Variable>> { None }

impl Default for TrapV1 {
    fn default() -> TrapV1 {
        TrapV1 {
            var: default_trapv1_var(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct V1 {
    pub pdu: Option<TrapV1>,
}

fn default_v1_pdu() -> Option<TrapV1> { None }

impl Default for V1 {
    fn default() -> V1 {
        V1 {
            pdu: default_v1_pdu(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Snmp {
    pub v1: Option<V1>,
}

fn default_snmp_v1() -> Option<V1> { None }

impl Default for Snmp {
    fn default() -> Snmp {
        Snmp {
            v1: default_snmp_v1(),
        }
    }
}
