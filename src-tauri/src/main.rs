// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::{Client, Method, header::{HeaderMap, HeaderName, HeaderValue}};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use std::str::FromStr;

// 1. The data coming FROM the Svelte UI
#[derive(Deserialize)]
struct RequestPayload {
    url: String,
    method: String,
    headers: HashMap<String, String>,
    body: String,
}

// 2. The data going TO the Svelte UI
#[derive(Serialize)]
struct ResponsePayload {
    status: u16,
    time_ms: u128,
    headers: HashMap<String, String>,
    body: String,
}

// 3. The Tauri Command that gets invoked by JavaScript
#[tauri::command]
async fn perform_request(request: RequestPayload) -> Result<ResponsePayload, String> {
    
    // Logs for checking headers
    // println!("----------------------------------------");
    // println!("🚀 RUST RECEIVED REQUEST: {} {}", request.method, request.url);
    // println!("📋 HEADERS: {:#?}", request.headers);
    // println!("----------------------------------------");


    let client = Client::new();
    
    // Safely parse the HTTP Method (GET, POST, etc.)
    let method = Method::from_str(&request.method).map_err(|e| e.to_string())?;
    
    // Build the Header map
    let mut header_map = HeaderMap::new();
    for (k, v) in request.headers {
        if let (Ok(name), Ok(value)) = (HeaderName::from_str(&k), HeaderValue::from_str(&v)) {
            header_map.insert(name, value);
        }
    }

    // Start a timer to measure response speed
    let start_time = Instant::now();

    // Construct the request
    let mut req_builder = client.request(method, &request.url).headers(header_map);
    
    // Attach the body if one exists
    if !request.body.is_empty() {
        req_builder = req_builder.body(request.body);
    }

    // Execute the request over the network
    let response = req_builder.send().await.map_err(|e| e.to_string())?;

    // Stop the timer
    let time_ms = start_time.elapsed().as_millis();
    let status = response.status().as_u16();
    
    // Extract response headers
    let mut res_headers = HashMap::new();
    for (k, v) in response.headers() {
        res_headers.insert(k.to_string(), v.to_str().unwrap_or("").to_string());
    }

    // Read the response body as plain text
    let body_text = response.text().await.map_err(|e| e.to_string())?;
    // println!("Response body received: {}", body_text);

    // Return the struct back across the IPC bridge
    Ok(ResponsePayload {
        status,
        time_ms,
        headers: res_headers,
        body: body_text,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![perform_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}