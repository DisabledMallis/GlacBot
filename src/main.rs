use std::{borrow::Borrow, env, io::Cursor};
use anyhow::Result;
use dotenv;
use futures::StreamExt;
use imageproc::{drawing::{Canvas, draw_cross, draw_cross_mut, draw_text, draw_hollow_circle_mut}, rgb_image};
use reqwest;
use tokio::task;
// use regex::Regex;
use twilight_gateway::{cluster::ShardScheme, Cluster, Event, Intents};
use twilight_http::Client;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageDecoder, ImageOutputFormat, Pixel, Rgb, RgbImage, Rgba};


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
                                        let mut writer: Vec<u8> = vec![];
                                        std::io::copy(&mut bytes, &mut writer)?;

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
    
                                http.create_message(msg.channel_id)
                                    .content(":warning: Now doing stuff!")?
                                    .exec()
                                    .await?; 
                                
                                let image = image::io::Reader::with_format(Cursor::new(&file), image::ImageFormat::Png);

                                let mut image = match image.decode() {
                                    Ok(image) => image,
                                    Err(_err) => {
                                        http.create_message(msg.channel_id)
                                        .content("Something happened while decoding provided image. Try a different one?")?
                                        .exec()
                                        .await?; 
                                        break;                                                 
                                    }
                                };

                                draw_hollow_circle_mut(&mut image,  (100, 100), 15, Rgba([255, 255, 255, 255]),);

                                let mut new_image = vec![];
                                image.write_to(&mut new_image, ImageOutputFormat::Png).unwrap();

                                http.create_message(msg.channel_id)
                                    .files(&[("name.png", &new_image)])
                                    .content("done")?
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