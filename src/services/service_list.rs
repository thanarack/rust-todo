use crate::db::{get_data, insert};

pub async fn get_service_list() -> Vec<&'static str> {
    return get_data().await;
}

pub async fn add_service_todo(data: &'static str) -> Result<(), std::io::Error> {
    let _ = insert(data).await;
    Ok(())
}
