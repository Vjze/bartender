use crate::resource::r#enum::{Btw, Error, Printers, Req};
use std::collections::HashMap;
use tiberius::{AuthMethod, Config};
use tokio::{
    net::TcpStream,
    time::{sleep, Duration},
};
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[derive(Debug, Clone, Default)]
pub struct InitData {
    pub librarie_id: String,
    pub printers: Vec<Printers>,
    pub btws: Vec<Btw>,
}
pub async fn init_all() -> Result<InitData, Error> {
    sleep(Duration::from_millis(1500)).await;
    let first = get_libraries().await;
    let mut data = InitData::default();
    match first {
        Ok(id) => {
            data.librarie_id = id.clone();
            let second = load_btws(id).await;
            match second {
                Ok(s) => {
                    data.btws = s;
                    let third = load_printers().await;
                    match third {
                        Ok(b) => {
                            data.printers = b;
                            Ok(data)
                        }
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}
pub async fn sql_init(sql: String) -> Result<String, Error> {
    let mut config = Config::new();
    config.host(sql);
    config.port(1433);
    config.database("TESTDATA");
    // config.authentication(AuthMethod::sql_server("cytest", "cytest"));
    config.authentication(AuthMethod::sql_server("hztest", "hztest"));
    config.trust_cert();
    let tcp = TcpStream::connect(config.get_addr()).await;
    match tcp {
        Ok(tcp) => {
            tcp.set_nodelay(true).unwrap();
            let client = tiberius::Client::connect(config, tcp.compat_write()).await;
            match client {
                Ok(_) => Ok("成功".to_string()),
                Err(_e) => Err(Error::SqlIDErr),
            }
        }
        Err(_e) => Err(Error::SqlIDErr),
    }
}
pub async fn get_libraries() -> Result<String, Error> {
    let url = "http://localhost/BarTender/api/v1/libraries".to_string();
    let client = reqwest::Client::new();
    let mut hm = HashMap::new();
    let res = client.get(url.clone()).send().await.unwrap();
    match res.status().is_success() {
        true => {
            for (key, val) in res.headers().into_iter() {
                hm.insert(
                    key.as_str().to_owned(),
                    val.to_str().ok().unwrap_or("").to_owned(),
                );
            }
            let req = Req {
                status: res.status().as_u16(),
                url: url.clone(),
                body: res.json().await.unwrap(),
                headers: hm,
            };
            let j = req.body.get(1);
            match j {
                Some(j) => {
                    let j = serde_json::to_string(&j["id"]).unwrap();
                    let id: String = j
                        .chars()
                        .map(|x| match x {
                            '"' => ' ',
                            '\\' => ' ',
                            _ => x,
                        })
                        .collect();
                    return Ok(id);
                }
                None => Err(Error::LibrariesNone),
            }
        }
        false => Err(Error::LibrariesNone),
    }
}
pub async fn load_printers() -> Result<Vec<Printers>, Error> {
    let url = "http://localhost/BarTender/api/v1/printers".to_string();
    let client = reqwest::Client::new();
    let mut hm = HashMap::new();
    let res = client.get(url.clone()).send().await.unwrap();
    match res.status().is_success() {
        true => {
            for (key, val) in res.headers().into_iter() {
                hm.insert(
                    key.as_str().to_owned(),
                    val.to_str().ok().unwrap_or("").to_owned(),
                );
            }
            let req = Req {
                status: res.status().as_u16(),
                url: url.clone(),
                body: res.json().await.unwrap(),
                headers: hm,
            };
            let server_printers = req.body.get("serverPrinters").unwrap();
            let x = server_printers.as_array().unwrap();
            let remote_printers = req.body.get("remotePrinters").unwrap();
            let y = remote_printers.as_array().unwrap();
            let mut printerlist = vec![];
            for i in x {
                let j = serde_json::to_string(i).unwrap();
                let s: String = j
                    .chars()
                    .map(|x| match x {
                        '"' => ' ',
                        '\\' => ' ',
                        _ => x,
                    })
                    .collect();
                let printer = Printers { printer: s };
                printerlist.push(printer);
            }
            for i in y {
                let j = serde_json::to_string(i).unwrap();
                let s: String = j
                    .chars()
                    .map(|x| match x {
                        '"' => ' ',
                        '\\' => ' ',
                        _ => x,
                    })
                    .collect();
                let printer = Printers { printer: s };
                printerlist.push(printer);
            }
            Ok(printerlist)
        }
        false => Err(Error::PrinterErr),
    }
}
pub async fn load_btws(id: String) -> Result<Vec<Btw>, Error> {
    let url = format!("http://localhost/BarTender/api/v1/libraries/{}", id);
    let client = reqwest::Client::new();
    let mut hm = HashMap::new();
    let res = client.get(url.clone()).send().await.unwrap();
    match res.status().is_success() {
        true => {
            for (key, val) in res.headers().into_iter() {
                hm.insert(
                    key.as_str().to_owned(),
                    val.to_str().ok().unwrap_or("").to_owned(),
                );
            }
            let req = Req {
                status: res.status().as_u16(),
                url: url.clone(),
                body: res.json().await.unwrap(),
                headers: hm,
            };
            let server_printers = req.body.get("contents").unwrap();
            let x = server_printers.as_array().unwrap();
            let mut btwlist = vec![];
            for i in x {
                let j = serde_json::to_string(i).unwrap();
                let s: String = j
                    .chars()
                    .map(|x| match x {
                        '"' => ' ',
                        '\\' => ' ',
                        _ => x,
                    })
                    .collect();
                let btws = Btw { btw: s };
                btwlist.push(btws);
            }

            Ok(btwlist)
        }
        false => Err(Error::BtwErr),
    }
}
