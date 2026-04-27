
use actix_web::{
    Responder, get,
    web::{self, Query},
};
use serde::{Deserialize, Serialize};
use sn_internal::db::{
    DBPool,
    db_group::{self, Group},
};

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/group").service(search));
}

#[derive(Debug, Deserialize)]
struct GroupQuery {
    search: String,
}

#[derive(Serialize, Default)]
struct GroupResponse {
    groups: Vec<Group>,
    err: String,
}

#[get("")]
async fn search(dbpool: web::Data<DBPool>, query_params: Query<GroupQuery>) -> impl Responder {
    let mut res = GroupResponse::default();
    match db_group::search(&dbpool, &query_params.search).await {
        Ok(ret) => res.groups = ret,
        Err(e) => res.err = e.to_string(),
    }

    web::Json(res)
}
