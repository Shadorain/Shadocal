use actix_web::{
    error, get,
    web::{self, Data, Json},
    Result,
};

use super::{format, Get, List, State};

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/tana").service(get).service(list));
}

#[get("/get")]
async fn get(data: Data<State>, Json(get): Json<Get>) -> Result<String> {
    println!("{:?}", get);
    data.get_event::<format::Tana>(get.cal_id, get.event_id)
        .await
        .map_err(|err| error::ErrorFailedDependency(err.to_string()))
}

#[get("/list")]
async fn list(data: Data<State>, Json(list): Json<List>) -> Result<String> {
    println!("{:?}", &list);
    let (start, end) = list.extract().ok_or(error::JsonPayloadError::Payload(
        error::PayloadError::EncodingCorrupted,
    ))?;
    data.list_events::<format::Tana>(start, end)
        .await
        .map_err(|err| error::ErrorFailedDependency(err.to_string()))
}
