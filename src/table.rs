use std::hash::Hash;

pub type ColumnIndex = usize;

#[derive(Clone)]
pub struct Column {
    name: String,
    data: ColumnData,
}

impl Column {
    pub fn new(name: String, data: ColumnData) -> Self {
        Self { name, data }
    }
}

#[derive(Clone)]
pub enum ColumnData {
    Text(String),
}

impl From<(&str, &Column)> for ColumnData {
    fn from(value: (&str, &Column)) -> Self {
        match value.1.data {
            ColumnData::Text(_) => ColumnData::Text(value.0.to_string()),
        }
    }
}

#[derive(Clone)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Table {
    pub fn new(name: String, columns: Vec<Column>) -> Self {
        Self { name, columns }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn get_column(&self, name: &String) -> Option<(ColumnIndex, Column)> {
        for (idx, col) in self.columns.iter().enumerate() {
            if name.eq(&col.name) {
                return Some((idx, col.clone()));
            }
        }

        return None;
    }
}

impl Eq for Table {}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Table {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

// struct DataRow {
//     // FIXME: make this String -> DataType
//     columns: HashMap<String, ColumnData>,
// }
