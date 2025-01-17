# Rust CRUD API with Axum

This project demonstrates a simple CRUD (Create, Read, Update, Delete) API built with Rust using the `axum` framework. It exposes endpoints for managing items in an in-memory storage system. Each item has a `name`, `description`, and a unique `id`.

## Features
- **Create**: Add new items to the storage.
- **Read**: Retrieve a list of all items.
- **Update**: Modify an existing item by its `id`.
- **Delete**: Remove an item by its `id`.

## Technologies Used
- **Rust**: Programming language used for the server.
- **Axum**: Web framework for building HTTP APIs in Rust.
- **Tokio**: Asynchronous runtime for building fast and reliable applications.
- **UUID**: Library for generating universally unique identifiers for each item.

## API Endpoints

### 1. Get all items
- **GET** `/items`
- Returns a list of all items in the storage.

#### Example Request
    ```bash
    curl -X GET http://127.0.0.1:3000/items
    Example Response
    
    [
      {
        "id": "f8a978ab-bd31-45f4-b040-8ac9037c5e50",
        "name": "Item 1",
        "description": "Description of item 1"
      },
      {
        "id": "ed0332fe-0e4d-4d43-a524-bc302b4b4734",
        "name": "Item 2",
        "description": "Description of item 2"
      }
    ]
### 2. Create a new item
    POST /items
    Creates a new item. The request body should contain a name and a description.
    Example Request
    
    curl -X POST http://127.0.0.1:3000/items \
      -H "Content-Type: application/json" \
      -d '{"name": "New Item", "description": "Description of the new item"}'
    Example Response
    
    {
      "id": "9b07c5be-0fa3-49f0-9ed4-618bbf4906f1",
      "name": "New Item",
      "description": "Description of the new item"
    }
###3. Update an existing item
    PUT /items/{id}
    Updates an existing item identified by id. The request body should contain the updated name and description.
    Example Request
    
    curl -X PUT http://127.0.0.1:3000/items/f8a978ab-bd31-45f4-b040-8ac9037c5e50 \
      -H "Content-Type: application/json" \
      -d '{"name": "Updated Item", "description": "Updated description"}'
    Example Response
    
    {
      "id": "f8a978ab-bd31-45f4-b040-8ac9037c5e50",
      "name": "Updated Item",
      "description": "Updated description"
    }
###4. Delete an item
    DELETE /items/{id}
    Deletes an item identified by id.
    Example Request
    
    curl -X DELETE http://127.0.0.1:3000/items/f8a978ab-bd31-45f4-b040-8ac9037c5e50
    Example Response
    true


Running the Project
Clone the repository:  git clone https://github.com/your-username/rust-crud-api.git
Change into the project directory:


cd rust-crud-api
Install dependencies and build the project:

cargo build
Run the application:


cargo run

The server will start on http://127.0.0.1:3000.
