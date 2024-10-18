use actix_web::{
    error, post,
    web::{self, Data, Json},
    Result,
};

use shadocal_lib::{
    format,
    types::{Get, List},
    State,
};

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/tana").service(get).service(list));
}

#[post("/get")]
async fn get(data: Data<State>, Json(get): Json<Get>) -> Result<String> {
    println!("{:?}", get);
    data.get_eventf::<format::Tana>(get.cal_id, get.event_id)
        .await
        .map_err(|err| error::ErrorFailedDependency(err.to_string()))
}

#[post("/list")]
async fn list(data: Data<State>, Json(list): Json<List>) -> Result<String> {
    println!("{:?}", &list);
    let (start, end) = list.extract().ok_or(error::JsonPayloadError::Payload(
        error::PayloadError::EncodingCorrupted,
    ))?;
    data.list_eventsf::<format::Tana>(start, end)
        .await
        .map_err(|err| error::ErrorFailedDependency(err.to_string()))
}
