use actix_web::{
    get,
    web::{self, Data, Query},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use shadocal_lib::State;

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(delete);
}

#[derive(Serialize, Deserialize)]
struct Delete {
    id: Option<String>,
}

#[get("/delete")]
async fn delete(data: Data<State>, Query(del): Query<Delete>) -> HttpResponse {
    println!("Deleting account: {:?} (None is all)", del.id);
    match data.delete(del.id).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
    .finish()
}
