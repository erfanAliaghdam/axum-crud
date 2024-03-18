use sqlx::{MySqlPool, Error};
use crate::core::models::TaskModel;


pub async fn get_all_tasks(data: &MySqlPool) -> Result<Vec<TaskModel>, Error> {
    let query = sqlx::query_as!(
        TaskModel,
        r#"SELECT id, title, created_at, updated_at FROM task"#
    );
    
    let tasks = query.fetch_all(data).await?;
    
    Ok(tasks)
}

pub async fn task_exists(data: &MySqlPool, id: &u32) -> Result<bool, Error> {
    // Execute a query to select the count of rows with the given task_id
    let result = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM task WHERE id = ?",
        id
    )
    .fetch_one(data)
    .await?;

    // If count is greater than zero, the task exists
    Ok(result > 0)
}

pub async fn get_task_by_id(data: &MySqlPool, id: &u32) -> Result<Vec<TaskModel>, Error> {
    let task_exists = task_exists(&data, &id).await?;
    if task_exists { 
        let query = sqlx::query_as!(
            TaskModel,
            r#"SELECT id, title, created_at, updated_at FROM task WHERE id = ?"#,
            id
        );
        
        let task = query.fetch_all(data).await?;
        
        Ok(task)
    } else {
        Err(Error::RowNotFound)
    }
}


pub async fn create_task(data: &MySqlPool, title: &str) -> Result<(), Error> {
    match sqlx::query!(
        r#"
        INSERT INTO task (title, created_at, updated_at)
        VALUES (?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        "#,
        title
    )
    .execute(data)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}


pub async fn update_task_by_id(data: &MySqlPool, id: &u32, title: &str) -> Result<(), Error> {
    let task_exists = task_exists(&data, &id).await?;
    if task_exists { 
        sqlx::query!(
            r#"
            UPDATE task
            SET title = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            title,
            id
        )
        .execute(data)
        .await?;

        Ok(())
    }else{
        Err(Error::RowNotFound)
    }
}


pub async fn delete_task_by_id(data: &MySqlPool, id: &u32) -> Result<(), Error> {
    let task_exists = task_exists(&data, &id).await?;
    if task_exists { 
        sqlx::query!(
            r#"
            DELETE FROM task
            WHERE id = ?
            "#,
            id
        )
        .execute(data)
        .await?;

        Ok(())
    }else{
        Err(Error::RowNotFound)
    }
}
