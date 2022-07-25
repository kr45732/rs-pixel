use super::error::Error;
use crate::RsPixel;
use serde_json::Value;

pub async fn username_to_uuid(rs_pixel: &RsPixel, username: &str) -> Result<Response, Error> {
    uuid_username(rs_pixel, username, false).await
}

pub async fn uuid_to_username(rs_pixel: &RsPixel, uuid: &str) -> Result<Response, Error> {
    uuid_username(rs_pixel, uuid, true).await
}

async fn uuid_username(
    rs_pixel: &RsPixel,
    uuid_username: &str,
    is_uuid: bool,
) -> Result<Response, Error> {
    match rs_pixel
        .config
        .client
        .get(match rs_pixel.config.minecraft_api_type {
            ApiType::Mojang => {
                if is_uuid {
                    format!(
                        "https://api.mojang.com/user/profiles/{}/names",
                        uuid_username
                    )
                } else {
                    format!(
                        "https://api.mojang.com/users/profiles/minecraft/{}",
                        uuid_username
                    )
                }
            }
            ApiType::Ashcon => {
                format!("https://api.ashcon.app/mojang/v2/user/{}", uuid_username)
            }
            ApiType::PlayerDb => {
                format!("https://playerdb.co/api/player/minecraft/{}", uuid_username)
            }
        })
        .send()
        .await
    {
        Ok(mut res_unwrap) => {
            let json = res_unwrap.body_json::<Value>().await.map_err(Error::from);

            if res_unwrap.status() == 200 {
                return match json {
                    Ok(json_unwrap) => Ok(match rs_pixel.config.minecraft_api_type {
                        ApiType::Mojang => Response {
                            username: (if is_uuid {
                                json_unwrap
                                    .as_array()
                                    .and_then(|v| v.last())
                                    .and_then(|v| v.get("name"))
                            } else {
                                json_unwrap.get("name")
                            })
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or("")
                            .to_string(),
                            uuid: (if is_uuid {
                                uuid_username
                            } else {
                                json_unwrap
                                    .get("id")
                                    .and_then(serde_json::Value::as_str)
                                    .unwrap_or("")
                            })
                            .to_string(),
                        },
                        ApiType::Ashcon => Response {
                            username: json_unwrap
                                .get("username")
                                .and_then(serde_json::Value::as_str)
                                .unwrap_or("")
                                .to_string(),
                            uuid: json_unwrap
                                .get("uuid")
                                .and_then(serde_json::Value::as_str)
                                .unwrap_or("")
                                .to_string()
                                .replace('-', ""),
                        },
                        ApiType::PlayerDb => Response {
                            username: json_unwrap
                                .get("data")
                                .and_then(|v| v.get("player"))
                                .and_then(|v| v.get("username"))
                                .and_then(serde_json::Value::as_str)
                                .unwrap_or("")
                                .to_string(),
                            uuid: json_unwrap
                                .get("data")
                                .and_then(|v| v.get("player"))
                                .and_then(|v| v.get("id"))
                                .and_then(serde_json::Value::as_str)
                                .unwrap_or("")
                                .to_string()
                                .replace('-', ""),
                        },
                    }),
                    Err(err) => Err(err),
                };
            }

            Err(Error::from((
                res_unwrap.status(),
                json.ok()
                    .as_ref()
                    .and_then(|json_unwrap| {
                        json_unwrap.get(match rs_pixel.config.minecraft_api_type {
                            ApiType::Mojang => "errorMessage",
                            ApiType::Ashcon => "reason",
                            ApiType::PlayerDb => "code",
                        })
                    })
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("Unknown fail cause")
                    .to_string(),
            )))
        }
        Err(err) => Err(Error::from(err)),
    }
}

#[derive(Debug)]
pub struct Response {
    pub username: String,
    pub uuid: String,
}

pub enum ApiType {
    Mojang,
    Ashcon,
    PlayerDb,
}
