trait Database {
    async fn insert_todo(&self, todo: Todo) -> Result<(), Error>;
}