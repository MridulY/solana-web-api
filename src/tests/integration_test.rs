#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use crate::handlers::{add_todo, get_task_by_id, update_todo, delete_todo};
    use crate::solana::client::{add_todo_on_solana, fetch_task_by_id, update_todo_on_solana, delete_todo_on_solana};

    #[actix_rt::test]
    async fn test_add_todo() {
        let text = "Test task";
        let result = add_todo_on_solana(text).await;
        assert!(result.is_ok(), "Task creation should succeed");
    }

    #[actix_rt::test]
    async fn test_fetch_task_by_id() {
        let task_id = "SomeTaskID"; // Use an actual task ID
        let result = fetch_task_by_id(task_id).await;
        assert!(result.is_ok(), "Fetching a task should succeed");
    }

    #[actix_rt::test]
    async fn test_update_todo() {
        let task_id = "SomeTaskID"; // Use an actual task ID
        let result = update_todo_on_solana(task_id, true).await;
        assert!(result.is_ok(), "Updating task should succeed");
    }

    #[actix_rt::test]
    async fn test_delete_todo() {
        let task_id = "SomeTaskID"; // Use an actual task ID
        let result = delete_todo_on_solana(task_id).await;
        assert!(result.is_ok(), "Deleting task should succeed");
    }
}
