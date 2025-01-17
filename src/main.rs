use axum::{
    routing::{get, post, put, delete},  // Added back post and delete
    extract::{Path, Json, State},
    Router, serve,
};
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

mod models;
mod storage;

use models::Item;
use storage::{read_storage, write_storage};

type SharedStorage = Arc<Mutex<Vec<Item>>>;

#[tokio::main]
async fn main() {
    let storage: SharedStorage = Arc::new(Mutex::new(read_storage()));

    let app = Router::new()
        .route("/items", get(get_items).post(create_item))
        .route("/items/{id}", put(update_item).delete(delete_item))  // Only using the new {id} syntax
        .with_state(storage);

    println!("Server running on http://127.0.0.1:3000");
    
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn get_items(state: State<SharedStorage>) -> Json<Vec<Item>> {
    let storage = state.lock().unwrap();
    Json(storage.clone())
}

async fn create_item(
    state: State<SharedStorage>,
    Json(payload): Json<Item>,  // Item structure will have no `id` in the body
) -> Json<Item> {
    let mut storage = state.lock().unwrap();
    let new_item = Item {
        id: Some(Uuid::new_v4()),  // Generate a new UUID for the item
        name: payload.name,
        description: payload.description,
    };
    storage.push(new_item.clone());
    write_storage(&storage);
    Json(new_item)  // Return the new item with the generated `id`
}


async fn update_item(
    state: State<SharedStorage>,
    Path(id): Path<Uuid>,
    Json(payload): Json<Item>,
) -> Json<Option<Item>> {
    let mut storage = state.lock().unwrap();
    
    // Find the item by id and update the name/description
    let updated_item = storage.iter_mut().find(|i| i.id == Some(id)).map(|item| {
        item.name = payload.name.clone();
        item.description = payload.description.clone();
        item.clone()  // Return the updated item
    });

    // If updated, write the updated storage to file
    if updated_item.is_some() {
        write_storage(&storage);
    }

    Json(updated_item)
}



async fn delete_item(
    state: State<SharedStorage>,
    Path(id): Path<Uuid>,
) -> Json<bool> {
    let mut storage = state.lock().unwrap();
    
    // Save the length before deletion
    let len_before = storage.len();
    
    // Retain all items except the one with the matching id
    storage.retain(|i| i.id != Some(id));
    
    // Check if the item was deleted by comparing lengths
    let deleted = len_before != storage.len();
    
    // If deleted, save the storage to the file
    if deleted {
        write_storage(&storage);
    }

    Json(deleted)
}
