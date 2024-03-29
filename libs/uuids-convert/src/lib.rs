use persistence::help_crates::mongodb::bson;

pub fn vec_bson_uuid_to_uuid(bson_uuids: Vec<bson::Uuid>) -> Vec<uuid::Uuid> {
    bson_uuids
        .into_iter()
        .map(|bson_uuid| bson_uuid.into())
        .collect()
}

pub fn vec_uuid_to_bson_uuid(uuids: Vec<uuid::Uuid>) -> Vec<bson::Uuid> {
    uuids.into_iter().map(|uuid| uuid.into()).collect()
}
