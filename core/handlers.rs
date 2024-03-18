use axum::{extract::{Path, State}, http::StatusCode, response::Json};
use axum::response::IntoResponse;
use sqlx::Error;
use serde_json::json;
use std::sync::Arc;
use crate::AppState;
use crate::core::serializers::{TaskCreateSerializer, TaskUpdateSerializer};
use crate::core::responses::Message;
use crate::core::repository::{
    create_task, 
    get_all_tasks, 
    update_task_by_id,
    get_task_by_id,
    delete_task_by_id
};


pub async fn hello_world_handler() -> impl IntoResponse  {
    let data = Message {
        status: "success",
        message: "Hello, world!",
    };
    let response = json!(&data);
    (StatusCode::OK, Json(response))
}


pub async fn todo_list_handler(
    State(data): State<Arc<AppState>>
) -> impl IntoResponse {

    // Attempt to fetch tasks from the repository
    match get_all_tasks(&data.db).await {
        Ok(tasks) => {
            // Process tasks as needed
            let res = json!({
                "status": "success",
                "message": "Tasks list retrieved successfully!",
                "data": tasks,
            });
            let response = json!(&res);
            (StatusCode::OK, Json(response))
        }
        Err(err) => {
            eprintln!("Failed to retrieve the tasks: {}", err);
            let res = Message {
                status: "error",
                message: "Something went wrong please try again later!."
            };
            let response = json!(&res);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }

}


pub async fn create_todo_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<TaskCreateSerializer>
) -> impl IntoResponse {
    match create_task(&data.db, &body.title).await {
        Ok(()) => {
            let data = json!({
                "status": "success",
                "message": "Task created successfully!."
            });
            let response = json!(&data);
            (StatusCode::CREATED, Json(response))
        },
        Err(err) => {
            eprintln!("Failed to create the task: {}", err);

            let res = Message {
                status: "error",
                message: "Something went wrong, please try again later."
            };
            let response = json!(&res);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }

}



pub async fn update_todo_handler(
    State(data): State<Arc<AppState>>,
    Path(id): Path<u32>,
    Json(body): Json<TaskUpdateSerializer>
) -> impl IntoResponse {
    match update_task_by_id(&data.db, &id, &body.title ).await {
        Ok(()) => {
            let data = json!({
                "status": "success",
                "message": "Task updated successfully!."
            });
            let response = json!(&data);
            (StatusCode::OK, Json(response))
        },
        Err(err) => {
            match err {
                Error::RowNotFound => {
                    let res = Message {
                        status: "error",
                        message: "Task not found.",
                    };
                    let response = json!(&res);
                    (StatusCode::NOT_FOUND, Json(response))
                }
                _ => {
                    eprintln!("Failed to update the task: {}", err);
                    let res = Message {
                        status: "error",
                        message: "Something went wrong, please try again later."
                    };
                    let response = json!(&res);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
                }
            }
        }
    }

}



pub async fn get_todo_detail_handler(
    State(data): State<Arc<AppState>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {

    match get_task_by_id(&data.db, &id).await {
        Ok(task) => {
            let res = json!({
                "status": "success",
                "message": "Task retrieved successfully!",
                "data": task,
            });
            let response = json!(&res);
            (StatusCode::OK, Json(response))
        }
        Err(err) => {
            match err {
                Error::RowNotFound => {
                    let res = Message {
                        status: "error",
                        message: "Task not found.",
                    };
                    let response = json!(&res);
                    (StatusCode::NOT_FOUND, Json(response))
                }
                _ => {
                    eprintln!("Failed to update the task: {}", err);
                    let res = Message {
                        status: "error",
                        message: "Something went wrong, please try again later."
                    };
                    let response = json!(&res);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
                }
            }
        }
    }

}



pub async fn delete_todo_handler(
    State(data): State<Arc<AppState>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {

    match delete_task_by_id(&data.db, &id).await {
        Ok(_) => {
            let res = json!({
                "status": "success",
                "message": "Task deleted successfully!",
            });
            let response = json!(&res);
            (StatusCode::OK, Json(response))
        }
        Err(err) => {
            match err {
                Error::RowNotFound => {
                    let res = Message {
                        status: "error",
                        message: "Task not found.",
                    };
                    let response = json!(&res);
                    (StatusCode::NOT_FOUND, Json(response))
                }
                _ => {
                    eprintln!("Failed to update the task: {}", err);
                    let res = Message {
                        status: "error",
                        message: "Something went wrong, please try again later."
                    };
                    let response = json!(&res);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
                }
            }
        }
    }

}