use tiberius::numeric::Numeric;

use crate::resource::{
    r#enum::{DataInfo, Error},
    util::client,
};

pub async fn run_query(sn: String, sql: String) -> Result<Vec<DataInfo>, Error> {
    let sn_check = examine(sn.clone(), sql.clone()).await;
    match sn_check {
        Ok(_) => {
            let sn = sn.clone();
            let mut list = vec![];
            let mut client = client(sql.clone()).await.unwrap();
            let queryvalues =
                "cus_pn, SNTitle, In_name, Inloss1, Reloss1, out_name, Inloss2, Reloss1, print_num";
            let query_ty = format!("where SN = '{}'", sn);
            let query_code = format!("select {} from  testdata_cy {}", queryvalues, query_ty);
            let stream = client.simple_query(query_code).await.unwrap();
            let result = stream.into_row().await.unwrap();
            match result {
                Some(r) => {
                    let sn = sn.clone();
                    let cus_pn = r.get::<&str, _>(0).unwrap().to_string();
                    let sntitle = r.get::<&str, _>(1).unwrap().to_string();
                    let in_name = r.get::<&str, _>(2).unwrap().to_string();
                    let inloss1 = r.get::<Numeric, _>(3).unwrap().to_string();
                    let reloss1 = r.get::<Numeric, _>(4).unwrap().to_string();
                    let out_name = r.get::<&str, _>(5).unwrap().to_string();
                    let inloss2 = r.get::<Numeric, _>(6).unwrap().to_string();
                    let reloss2 = r.get::<Numeric, _>(7).unwrap().to_string();
                    let print_num = r.get::<i32, _>(8).unwrap();
                    let datainfo = DataInfo {
                        sn,
                        cus_pn,
                        sntitle,
                        in_name,
                        inloss1,
                        reloss1,
                        out_name,
                        inloss2,
                        reloss2,
                        print_num,
                    };
                    list.push(datainfo);
                    Ok(list)
                }
                None => Err(Error::SnNone),
            }
        }
        Err(e) => Err(e),
    }
}
async fn examine(sn: String, sql: String) -> Result<bool, Error> {
    if sn.is_empty() {
        Err(Error::Inputisempty)
    } else {
        let mut client = client(sql).await.unwrap();
        let query_ty = format!("where SN = '{}'", sn);
        let query_code = format!("select print_num from  testdata_cy {}", query_ty);
        let stream = client.simple_query(query_code).await.unwrap();
        let result = stream.into_row().await.unwrap();
        match result {
            Some(row) => {
                let status = row.get::<i32, _>(0).unwrap();
                if status == 0 {
                    Ok(true)
                } else {
                    Err(Error::NoFirst)
                }
            }
            None => Err(Error::SnNone),
        }
    }
}
