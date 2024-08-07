use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DataFrame<T> {
    rows: Vec<Vec<T>>,
    row_len: usize,
    col_len: usize,
    columns: Option<Vec<String>>,
}
impl<T> fmt::Display for DataFrame<T>
where
    T: ToString,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn format_row<T>(row: Vec<T>) -> String
        where
            T: ToString,
        {
            let mut row: String = row
                .iter()
                .map(|r| format!("{: >13}", r.to_string()))
                .collect();
            row.truncate(13);
            row
        }
        let body: String = self
            .rows
            .iter()
            .map(|r| {
                format!(
                    "{}\n",
                    r.iter()
                        .map(
                            // |v| {
                            // let mut res = format!("{: >13}", v.to_string());
                            // res.truncate(13);
                            // res
                            format_row //})
                        )
                        .collect::<Vec<String>>()
                        .join("|")
                )
            })
            .collect();

        let header = match &self.columns {
            Some(cols) => cols
                .iter()
                .map(|v| format!("{: >13}", v.to_string()))
                .collect::<Vec<String>>()
                .join("|"),
            None => String::from(""),
        };
        let separator = "-".repeat(body.lines().next().unwrap_or("").len());

        write!(f, "{}\n{}\n{}{}", header, separator, body, separator)
    }
}

#[derive(Debug)]
pub enum SQLError {
    EmptyDataFrame,
    MismatchedColumns,
}

impl fmt::Display for SQLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SQLError::EmptyDataFrame => write!(f, "One or more dataframes is empty."),
            SQLError::MismatchedColumns => write!(f, "Columns do not match."),
        }
    }
}

impl Error for SQLError {}

impl<T> DataFrame<T>
where
    T: Clone,
{
    pub fn new(rows: Option<Vec<Vec<T>>>, columns: Option<Vec<String>>) -> DataFrame<T> {
        let rows = match rows {
            Some(rs) => rs,
            None => vec![],
        };
        let col_len = rows.len();
        let row_len = if col_len > 0 { rows[0].len() } else { 0 };
        return DataFrame {
            col_len,
            row_len,
            rows,
            columns,
        };
    }

    pub fn union_all(&mut self, right: &mut DataFrame<T>) -> Result<&DataFrame<T>, Box<dyn Error>> {
        if self.col_len == 0 || right.col_len == 0 {
            return Err(Box::new(SQLError::EmptyDataFrame));
        } else if self.row_len != right.row_len {
            return Err(Box::new(SQLError::MismatchedColumns));
        }
        self.rows.append(&mut right.rows);
        self.col_len = self.col_len + right.col_len;
        Ok(self)
    }
}
