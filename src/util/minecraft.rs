use super::error::RsPixelError;
use crate::RsPixel;
use serde_json::Value;

pub async fn username_to_uuid(
    rs_pixel: &RsPixel,
    username: &str,
) -> Result<MinecraftResponse, RsPixelError> {
    uuid_username(rs_pixel, username, false).await
}

pub async fn uuid_to_username(
    rs_pixel: &RsPixel,
    uuid: &str,
) -> Result<MinecraftResponse, RsPixelError> {
    uuid_username(rs_pixel, uuid, true).await
}

async fn uuid_username(
    rs_pixel: &RsPixel,
    uuid_username: &str,
    is_uuid: bool,
) -> Result<MinecraftResponse, RsPixelError> {
    match rs_pixel
        .config
        .client
        .get(match rs_pixel.config.minecraft_api_type {
            MinecraftApiType::Mojang => {
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
            MinecraftApiType::Ashcon => {
                format!("https://api.ashcon.app/mojang/v2/user/{}", uuid_username)
            }
            MinecraftApiType::PlayerDb => {
                format!("https://playerdb.co/api/player/minecraft/{}", uuid_username)
            }
        })
        .send()
        .await
    {
        Ok(mut res_unwrap) => {
            let json = res_unwrap
                .body_json::<Value>()
                .await
                .map_err(|err| RsPixelError::from(err));

            if res_unwrap.status() == 200 {
                return match json {
                    Ok(json_unwrap) => Ok(match rs_pixel.config.minecraft_api_type {
                        MinecraftApiType::Mojang => MinecraftResponse {
                            username: (if is_uuid {
                                json_unwrap
                                    .as_array()
                                    .and_then(|v| v.last())
                                    .and_then(|v| v.get("name"))
                            } else {
                                json_unwrap.get("name")
                            })
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                            uuid: (if is_uuid {
                                uuid_username
                            } else {
                                json_unwrap.get("id").and_then(|v| v.as_str()).unwrap_or("")
                            })
                            .to_string(),
                        },
                        MinecraftApiType::Ashcon => MinecraftResponse {
                            username: json_unwrap
                                .get("username")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            uuid: json_unwrap
                                .get("uuid")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string()
                                .replace("-", ""),
                        },
                        MinecraftApiType::PlayerDb => MinecraftResponse {
                            username: json_unwrap
                                .get("data")
                                .and_then(|v| v.get("player"))
                                .and_then(|v| v.get("username"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            uuid: json_unwrap
                                .get("data")
                                .and_then(|v| v.get("player"))
                                .and_then(|v| v.get("id"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string()
                                .replace("-", ""),
                        },
                    }),
                    Err(err) => Err(err),
                };
            }

            Err(RsPixelError::from((
                res_unwrap.status(),
                json.ok()
                    .as_ref()
                    .and_then(|json_unwrap| {
                        json_unwrap.get(match rs_pixel.config.minecraft_api_type {
                            MinecraftApiType::Mojang => "errorMessage",
                            MinecraftApiType::Ashcon => "reason",
                            MinecraftApiType::PlayerDb => "code",
                        })
                    })
                    .and_then(|cause| cause.as_str())
                    .unwrap_or("Unknown fail cause")
                    .to_string(),
            )))
        }
        Err(err) => Err(RsPixelError::from(err)),
    }
}

#[derive(Debug)]
pub struct MinecraftResponse {
    pub username: String,
    pub uuid: String,
}

pub enum MinecraftApiType {
    Mojang,
    Ashcon,
    PlayerDb,
}
