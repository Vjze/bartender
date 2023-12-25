use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Default)]
pub struct DataInfo {
    pub sn: String,
    pub cus_pn: String,
    pub sntitle: String,
    pub in_name: String,
    pub inloss1: String,
    pub reloss1: String,
    pub out_name: String,
    pub inloss2: String,
    pub reloss2: String,
    pub print_num: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Req {
    pub url: String,
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: serde_json::Value,
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum SqlSelect {
    Local,
    #[default]
    Net,
    Test,
}
impl SqlSelect {
    pub const ALL: [SqlSelect; 3] = [SqlSelect::Local, SqlSelect::Net, SqlSelect::Test];
}
impl Display for SqlSelect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SqlSelect::Local => {
                    "127.0.0.1"
                }
                SqlSelect::Net => {
                    "192.168.10.254"
                }
                SqlSelect::Test => {
                    "192.168.2.189"
                }
            }
        )
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Printers {
    pub(crate) printer: String,
}
impl Default for Printers {
    fn default() -> Self {
        Self {
            printer: Default::default(),
        }
    }
}

impl std::fmt::Display for Printers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.printer)
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Btw {
    pub(crate) btw: String,
}
impl Default for Btw {
    fn default() -> Self {
        Self {
            btw: Default::default(),
        }
    }
}

impl std::fmt::Display for Btw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.btw)
    }
}
#[derive(Debug, Clone, Default)]
pub enum Error {
    #[default]
    Inputisempty,
    SnNone,
    NoFirst,
    SqlIDErr,
    LibrariesNone,
    UpdataErr,
    PrintErr,
    PrinterErr,
    BtwErr,
    BtwNoSelect,
    PrinterNoSelect,
    InitNotDone,
}
