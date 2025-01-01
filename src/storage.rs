use std::{collections::HashSet, path::PathBuf};

use bytes::Bytes;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::{
    qp::Plan,
    table::{Column, ColumnData, Table},
    AppError, AppResult,
};

pub enum ExecutionResult {
    Select(Bytes),
    Insert(AppResult<()>),
}

/// Contains shared state among all connections.
// FIXME: implement multi-tenancy separation
#[derive(Clone)]
pub struct Storage {
    base_db_dir: PathBuf,
    tables: HashSet<Table>,
}

impl Storage {
    pub fn new() -> Self {
        // FIXME: implement schema updates

        let columns = vec![
            Column::new("id".to_string(), ColumnData::Text(String::new())),
            Column::new("name".to_string(), ColumnData::Text(String::new())),
        ];

        let table = Table::new("mytable".to_string(), columns);
        let mut tables = HashSet::new();
        tables.insert(table);

        let base_db_dir = std::env::current_dir()
            .unwrap()
            .join("data")
            .join("database");
        std::fs::create_dir_all(&base_db_dir).unwrap();

        Self {
            base_db_dir,
            tables,
        }
    }

    pub async fn execute(&self, plan: Plan) -> AppResult<ExecutionResult> {
        match plan {
            Plan::Select {
                ref cols,
                ref table,
            } => {
                let table = self
                    .tables
                    .get(&Table::new(table.to_string(), vec![]))
                    .unwrap();

                let table_cols = cols
                    .iter()
                    .map(|col| {
                        table
                            .get_column(col)
                            .ok_or(AppError::InvalidSchema(format!("column {col} not found")))
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let mut rows = vec![];
                let table_file = tokio::fs::OpenOptions::new()
                    .read(true)
                    .open(self.base_db_dir.join(table.name()))
                    .await?;
                let mut reader = BufReader::new(table_file);
                let mut line = String::new();
                while let Ok(_) = reader.read_line(&mut line).await {
                    let split = line.split(",").collect::<Vec<_>>();
                    let row = table_cols
                        .iter()
                        .map(|(idx, col)| (split[*idx], col).into())
                        .collect::<Vec<ColumnData>>();
                    rows.push(row);

                    line.clear();
                }

                Ok(ExecutionResult::Select(Bytes::new()))
            }

            Plan::Insert { values, table } => {
                // FIXME: check values before inserting
                let mut table_file = tokio::fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(self.base_db_dir.join(table))
                    .await?;
                table_file.write_all(values.join(",").as_bytes()).await?;

                Ok(ExecutionResult::Insert(Ok(())))
            }
        }
    }
}
