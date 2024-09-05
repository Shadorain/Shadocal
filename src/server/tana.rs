use actix_web::{error, get, web, Result};

use super::{format, Get, List, State};

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/tana").service(get).service(list));
}

#[get("/get/{cal_id}/{event_id}")]
async fn get(_: web::Data<State>, get: web::Path<Get>) -> Result<String> {
    println!("{:?}", get);
    todo!()
}

#[get("/list")]
async fn list(data: web::Data<State>, list: web::Json<List>) -> Result<String> {
    println!("{:?}", &list);
    let (start, end) = list.extract().ok_or(error::JsonPayloadError::Payload(
        error::PayloadError::EncodingCorrupted,
    ))?;
    data.get_events::<format::Tana>(start, end)
        .await
        .map_err(|err| error::ErrorFailedDependency(err.to_string()))
}
