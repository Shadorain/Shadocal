use actix_web::{
    error, post,
    web::{self, Data, Json},
    Result,
};

use super::{format::Tana, Format, Get, List, State};

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/tana").service(get).service(list));
}

#[post("/get")]
async fn get(data: Data<State>, Json(get): Json<Get>) -> Result<String> {
    println!("{:?}", get);
    Tana::format(
        data.get_event(get.cal_id, get.event_id)
            .await
            .map_err(|err| error::ErrorFailedDependency(err.to_string()))?,
    )
    .ok_or(error::ErrorFailedDependency("Couldn't format event"))
}

#[post("/list")]
async fn list(data: Data<State>, Json(list): Json<List>) -> Result<String> {
    println!("{:?}", &list);
    let (start, end) = list.extract().ok_or(error::JsonPayloadError::Payload(
        error::PayloadError::EncodingCorrupted,
    ))?;
    Tana::format_list(
        data.list_events(start, end)
            .await
            .map_err(|err| error::ErrorFailedDependency(err.to_string()))?,
    )
    .ok_or(error::ErrorFailedDependency("Couldn't format event"))
}
