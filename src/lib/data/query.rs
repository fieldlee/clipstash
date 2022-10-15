use super::models;
use crate::data::{DataError,DatabasePool};
use crate::ShortCode;
use sqlx::Row;
use crate::web::api::ApiKey;

type Result<T> = std::result::Result<T, DataError>;

pub async fn increase_hit_count(shortcode: &ShortCode, hits: u32, pool: &DatabasePool) -> Result<()> {

    let shortcode = shortcode.as_str();
    sqlx::query!(
        "Update clips SET hits = hits + ? WHERE shortcode = ?",
        hits, 
        shortcode
    ).execute(pool).await?;
    Ok(())

}

pub async fn get_clip<M: Into<models::GetClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<models::Clip> {

    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(sqlx::query_as!(
        models::Clip,
        "SELECT * FROM clips WHERE shortcode = ?",
        shortcode    
    ).fetch_one(pool).await?)
}


pub async fn new_clip<M: Into<models::NewClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<models::Clip> {

    let model = model.into();
    let _ = sqlx::query!(
        r#"INSERT INTO clips (
            clip_id,
            shortcode,
            content,
            title,
            posted,
            expires,
            password,
            hits)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0
    )
    .execute(pool).await?;

    get_clip(model.shortcode, pool).await
}

pub async fn update_clip<M: Into<models::UpdateClip>> (
    model: M,
    pool: &DatabasePool,
) -> Result<models::Clip> {

    let model = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET 
            content = ?, 
            expires = ?, 
            password = ?, 
            title = ?
            WHERE shortcode = ?"#,

        model.content,
        model.expires,
        model.password,
        model.title,
        model.shortcode
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}


pub async fn save_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<ApiKey> 
{
    let bytes = api_key.clone().into_inner();
    let _ = sqlx::query!("INSERT INTO api_keys (api_key) VALUES (?)", bytes)
        .execute(pool)
        .await
        .map(|_|())?;
    Ok(api_key)
}

pub enum RevocationStatus {
    Revoked,
    NotFound
}

pub async fn revoke_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<RevocationStatus>
{
    let bytes = api_key.clone().into_inner();
    Ok(
        sqlx::query!("DELETE FROM api_keys WHERE api_key = ?", bytes)
            .execute(pool)
            .await
            .map(|result| match result.rows_affected() {
                0 => RevocationStatus::NotFound,
                _ => RevocationStatus::Revoked
            })?,
    )
}


pub async fn api_key_is_valid(api_key: ApiKey, pool: &DatabasePool) -> Result<bool>
{
    let bytes = api_key.clone().into_inner();
    Ok(
        sqlx::query("SELECT COUNT(api_key) FROM api_keys where api_key = ?")
            .bind(bytes)
            .fetch_one(pool)
            .await
            .map(|row| {
                let count: u32 = row.get(0);
                count > 0
            })?,
    )
}

pub async fn delete_expired(pool: &DatabasePool) -> Result<u64> {
    Ok(
    sqlx::query!(r#"DELETE FROM clips WHERE now() > expires"#)
        .execute(pool)
        .await?
        .rows_affected()
    )
}
