use anyhow::Result;
use axum::{
    extract::{Extension, Path},
    http::{HeaderMap, HeaderValue, StatusCode},
    routing::get,
    Router,
};
use bytes::Bytes;
use image::ImageOutputFormat;
use lru::LruCache;
use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::{add_extension::AddExtensionLayer, compression::CompressionLayer};
use tracing::{info, instrument};

mod engine;
mod pb;

use engine::Photon;
use pb::*;

use crate::engine::Engine;

// use anyhow::{Ok, Result};
// use axum::{
//     extract::{Extension, Path},
//     http::{HeaderMap, HeaderValue, StatusCode},
//     routing::get,
//     Router,
// };
// use bytes::Bytes;
// // use engine::Photon;
// use image::ImageOutputFormat;
// use lru::LruCache;
// use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
// use serde::Deserialize;
// use std::{
//     collections::hash_map::DefaultHasher,
//     convert::TryInto,
//     hash::{Hash, Hasher},
//     sync::Arc,
// };
// use tokio::sync::Mutex;
// use tower::ServiceBuilder;
// use tracing::{info, instrument};

// mod engine;
// mod pb;

// use engine::Photon;
// use pb::*;
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}
// 定义Cache类型 Arc容器 > 锁 > Lru缓存
type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // 初始化自定义类型Cache
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(1024)));

    // 初始化路由
    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(ServiceBuilder::new().layer(Extension(cache)));
    // 监听地址
    let addr = "127.0.0.1:3001".parse().unwrap();
    print_test_url("https://images.pexels.com/photos/1562477/pexels-photo-1562477.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260");
    info!("Listening on {}", addr);
    // 启动服务
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// async fn generate(
//     Path(Params { spec, url }): Path<Params>,
//     Extension(cache): Extension<Cache>,
// ) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
//     let spec: ImageSpec = spec
//         .as_str()
//         .try_into()
//         .map_err(|_| StatusCode::BAD_REQUEST)?;
//     let url = percent_decode_str(&url).decode_utf8_lossy();

//     let data = retrieve_image(&url, cache)
//         .await
//         .map_err(|_| StatusCode::BAD_REQUEST)?;

//     let mut headers = HeaderMap::new();
//     headers.insert("Content-Type", HeaderValue::from_static("image/jpeg"));
//     Ok((headers, data.to_vec()))
// }
async fn generate(
    Path(Params { spec, url }): Path<Params>,
    Extension(cache): Extension<Cache>,
) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let url: &str = &percent_decode_str(&url).decode_utf8_lossy();
    let data = retrieve_image(url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 使用 image engine 处理
    let mut engine: Photon = data
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    engine.apply(&spec.specs);
    // TODO: 这里目前类型写死了，应该使用 content negotiation
    let image = engine.generate(ImageOutputFormat::Jpeg(85));

    info!("Finished processing: image size {}", image.len());
    let mut headers = HeaderMap::new();

    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));
    Ok((headers, image))
}

#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();

    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        Some(v) => {
            info!("Match Cache  {}", key);
            v.to_owned()
        }
        None => {
            info!("Retrieve url");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        }
    };
    Ok(data)
}

// 调试辅助函数
fn print_test_url(url: &str) {
    use std::borrow::Borrow;
    let spec1 = Spec::new_resize(500, 800, resize::SampleFilter::CatmullRom);
    let spec2 = Spec::new_watermark(20, 20);
    let spec3 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
    let s: String = image_spec.borrow().into();
    let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
    println!("test url: http://localhost:3000/image/{}/{}", s, test_image);
}
