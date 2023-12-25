use chrono::Local;
use serde_json::json;
use std::collections::HashMap;

use crate::resource::{
    r#enum::{DataInfo, Error, Req},
    util::client,
};

pub async fn print(
    list: Vec<DataInfo>,
    id: String,
    btw: String,
    printer: String,
    float: u32,
) -> Result<bool, Error> {
    let datainfo = list.get(0).unwrap().clone();
    let sn = datainfo.sn;
    let pn = datainfo.cus_pn;
    let sntitle = datainfo.sntitle;
    let in_name = datainfo.in_name;
    let out_name = datainfo.out_name;
    let inloss1_c = &datainfo.inloss1[..3];
    let inloss2_c = &datainfo.inloss2[..3];
    let pt: usize = if float == 2 { 5 } else { 2 };
    let printer = printer.clone();
    let reloss1_c = &datainfo.reloss1[..pt];
    let reloss2_c = &datainfo.reloss2[..pt];
    let btw = &btw.clone()[1..];
    let data = json!({
                            // 模版库的ID
                        "LibraryID": format!("{}",id),
                            // 模版的绝对路径,与相对路径二者选其一
                        // "AbsolutePath": "global_test.btw",
                            // 模版的相对路径，例如：Automotive/AIAG/B-10/BMW.btw
                        "relativePath": format!("{}",btw),

                            // 打印机名称
                        "printer": format!("{}",printer),
                            // 起始位置（一般不传，从参数中拿掉）
                        // "StartingPosition": 1,
                            // /打印份数
                        "Copies": 1,
                            // 自增序列
                        "SerialNumbers": format!("{}",0),
                            // 老版软件设置参数接口
                        // "DataEntryControls": {
                            // 新版软件设置参数接口
                        "namedDataSources": {
                            "SN":format!("{}",sn),
                            "PN":format!("{}",pn),
                            "TITLE":format!("{}",sntitle),
                            "INLOSS1":format!("≤{}dB",inloss1_c),
                            "INLOSS2":format!("≤{}dB",inloss2_c),
                            "JK1":format!("{}",in_name),
                            "JK2":format!("{}",out_name),
                            "RELOSS1":format!("≥{}dB",reloss1_c),
                            "RELOSS2":format!("≥{}dB",reloss2_c),
    }
        });
    println!("data = {}", data);

    let worker_thread = tokio::spawn(async move {
        let url = "http://localhost/BarTender/api/v1/print".to_string();
        let client = reqwest::Client::new();
        let mut hm = HashMap::new();
        let res = client.post(url.clone()).json(&data).send().await.unwrap();
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
                let b = req.body.get("success").unwrap().as_bool().unwrap();
                println!("message = {:?}", req.body);
                b
            }
            false => false,
        }
    });
    match worker_thread.await {
        Ok(b) => match b {
            true => Ok(b),
            false => Err(Error::PrintErr),
        },

        Err(_e) => Err(Error::PrintErr),
    }
}

pub async fn updata(list: Vec<DataInfo>, sql: String) -> Result<String, Error> {
    let data = list.get(0).unwrap();
    let print_num = data.print_num.clone();
    let sn = data.sn.clone();
    let new_print_num = print_num + 1;
    let date = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let mut client = client(sql).await.unwrap();
    let stream = client
        .execute(
            format!(
                "UPDATE testdata_cy set print_num = '{}', print_date = '{}' WHERE sn = '{}'",
                new_print_num, date, sn
            ),
            &[&1i32],
        )
        .await;
    match stream {
        Ok(_) => Ok("已完成！".to_string()),
        Err(_) => Err(Error::UpdataErr),
    }
}
