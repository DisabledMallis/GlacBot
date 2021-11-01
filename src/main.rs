use std::{env, io::Cursor};
use anyhow::Result;
use dotenv;
use futures::StreamExt;
use imageproc::{drawing::draw_line_segment_mut};
use reqwest;
use tokio::task;
// use regex::Regex;
use twilight_gateway::{cluster::ShardScheme, Cluster, Event, Intents};
use serde::{Serialize,Deserialize};
use twilight_http::Client;
use image::{ImageOutputFormat, Rgba, io::Reader};

#[derive(Debug)]
pub enum CustomError {
    RequestError(reqwest::Error),
    CopyError,
    OtherError
}

async fn download_user_skin(user_id: String) -> Result<Vec<u8>,CustomError> {
    let user_id = format!("https://crafatar.com/skins/{}",user_id);
    let user_skin = reqwest::get(user_id).await.map_err(|e| CustomError::RequestError(e))?;

    let mut bytes = Cursor::new(user_skin.bytes().await.map_err(|_| CustomError::OtherError)?);
    let mut writer: Vec<u8> = vec![];
    std::io::copy(&mut bytes, &mut writer).map_err(|_| CustomError::CopyError)?;

    Ok(writer)
}

fn download_skin<'a>(bytes: &'a Vec<u8>) -> Reader<Cursor<&'a Vec<u8>>> {
    image::io::Reader::with_format(Cursor::new(bytes), image::ImageFormat::Png)
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct NameChange {
    name: String,
    #[serde(rename="changedToAt")]
    changed_to_at: Option<usize>
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PlayerMeta {
    pub name_history: Vec<NameChange>
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PlayerInfo {
    pub meta: PlayerMeta,
    pub username: String,
    pub id: String,
    pub raw_id: String,
    pub avatar: String,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct RequestUserData {
    pub player: PlayerInfo
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct UserInfoRequest {
  pub code: String,
  pub message: String,
  pub success: bool,
  pub data: RequestUserData
}  

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_filename(".env").ok();
    let token = env::var("TOKEN")?;

	let cluster = Cluster::builder(&*token, Intents::GUILD_MESSAGES)
		.shard_scheme(ShardScheme::Auto)
		.build()
		.await?;

	let cluster_spawn = cluster.0.clone();
	task::spawn(async move {
		cluster_spawn.up().await;
	});

	let http = Client::new((&*token).to_string());

    // let cache =
    // InMemoryCache::builder().resource_types(ResourceType::all()).build();

    let mut events = cluster.1;
	while let Some((_shard_id, event)) = events.next().await {
		match event {
			Event::MessageCreate(msg) => {
                if !msg.author.bot {
                    let prefixed_6 = msg.content.get(0..7);

                    if prefixed_6.is_some() && prefixed_6.unwrap().to_lowercase() == "glacify" {

                        if !msg.attachments.is_empty() {    
                            for attachment in &msg.attachments {
                                if !attachment.filename.ends_with(".png") {
                                    http.create_message(msg.channel_id)
                                        .content("Only .PNG files can be glacced")?
                                        .exec()
                                        .await?;
                                    break;
                                }
    

                                let file = match reqwest::get(&attachment.url).await {
                                    Ok(res) => {
                                        let mut bytes = Cursor::new(res.bytes().await?);
                                        let mut writer: Vec<u8> = vec![
                                            
                                        ];
                                        match std::io::copy(&mut bytes, &mut writer) {
                                            Ok(_) => {},
                                            Err(_) => {
                                                http.create_message(msg.channel_id)
                                                .content(":x:")?
                                                .exec()
                                                .await?;
                                                break;
                                            },
                                        }

                                        writer
                                    }
                                    Err(_err) => {
                                        http.create_message(msg.channel_id)
                                        .content("Failed to download the desired file.")?
                                        .exec()
                                        .await?; 
                                        break;                                    
                                    }
                                };
    
                                let image = download_skin(&file);

                                let mut image = match image.decode() {
                                    Ok(image) => image,
                                    Err(_err) => {
                                        http.create_message(msg.channel_id)
                                        .content(":x:")?
                                        .exec()
                                        .await?; 
                                        break;                                                 
                                    }
                                };

                                draw_line_segment_mut(&mut image, (37 as f32,11 as f32), (50 as f32,11 as f32), Rgba([0,0,0,100]));
                                draw_line_segment_mut(&mut image, (41 as f32,12 as f32), (42 as f32,12 as f32), Rgba([0,0,0,100]));
                                draw_line_segment_mut(&mut image, (45 as f32,12 as f32), (46 as f32,12 as f32), Rgba([0,0,0,100]));


                                let mut new_image = vec![];
                                image.write_to(&mut new_image, ImageOutputFormat::Png).unwrap();

                                http.create_message(msg.channel_id)
                                    .files(&[(attachment.filename.as_str(), &new_image)])
                                    .exec()
                                    .await?; 
    
                            }
                        } else if let Some(mc_name) = msg.content.split(" ").nth(1) {
                            http.create_message(msg.channel_id)
                                .content(&format!("glacifying {}", mc_name))?
                                .exec()
                                .await?;

                            let data = reqwest::get(format!("https://playerdb.co/api/player/minecraft/{}",mc_name)).await?;

                            if let Ok(user_data) = data.json::<UserInfoRequest>().await {
                                let user_id = user_data.data.player.id.clone();

                                http.create_message(msg.channel_id)
                                    .content(&format!("Got uuid: {}", user_id))?
                                    .exec()
                                    .await?;

                                let user_skin = download_user_skin(user_id).await.unwrap();
                                let image = download_skin(&user_skin);

                                let mut image = match image.decode() {
                                    Ok(image) => image,
                                    Err(_err) => {
                                        http.create_message(msg.channel_id)
                                        .content(":x:")?
                                        .exec()
                                        .await?; 
                                        break;                                                 
                                    }
                                };

                                draw_line_segment_mut(&mut image, (37 as f32,11 as f32), (50 as f32,11 as f32), Rgba([0,0,0,100]));
                                draw_line_segment_mut(&mut image, (41 as f32,12 as f32), (42 as f32,12 as f32), Rgba([0,0,0,100]));
                                draw_line_segment_mut(&mut image, (45 as f32,12 as f32), (46 as f32,12 as f32), Rgba([0,0,0,100]));


                                let mut new_image = vec![];
                                image.write_to(&mut new_image, ImageOutputFormat::Png).unwrap();

                                http.create_message(msg.channel_id)
                                    .files(&[(&format!("{}-modified.png", user_data.data.player.username), &new_image)])
                                    .exec()
                                    .await?;
                            }
                        }      
                    }           
                }
            }
			_ => {continue;}
		}
	}

	Ok(())
}