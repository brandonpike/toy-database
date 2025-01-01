use bytes::Bytes;

use crate::{storage::Storage, AppError, AppResult};

#[derive(Debug)]
pub enum Plan {
    Select { cols: Vec<String>, table: String },
    Insert { values: Vec<String>, table: String },
}

/// Processes queries and returns result back to the connection router.
#[derive(Clone)]
pub struct QueryProcessor {
    storage: Storage,
}

impl QueryProcessor {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    // SELECT * from mytable;
    // SELECT Id, Name from mytable;
    //
    // INSERT into mytable values (values)
    pub async fn process(&self, query: String) -> AppResult<Bytes> {
        let query = query.trim();
        let split = query
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let plan = match split[0].to_lowercase().as_str() {
            "select" => {
                if split.len() == 2 {
                    return Ok(Bytes::from(split[1].clone()));
                }
                // Ensure valid select query length
                if split.len().ne(&4) {
                    return Err(AppError::InvalidQuery(query.to_string()));
                }

                // FIXME: split on comma and clear whitespace for multi col
                // selects
                let cols = vec![split[1].to_string()];
                let table = split[3].to_string();

                Plan::Select { cols, table }
            }
            "insert" => {
                // Ensure valid insert query length
                if split.len().ne(&5) {
                    return Err(AppError::InvalidQuery(query.to_string()));
                }

                // FIXME: impl value types and checking
                let values = split[4]
                    .replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                let table = split[2].to_string();

                Plan::Insert { values, table }
            }
            _ => todo!(),
        };

        let execute = self.storage.execute(plan).await?;
        Ok(Bytes::new())
    }
}
