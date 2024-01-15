use std::fmt::{Display, Formatter, Error};

#[derive (Debug)]
pub struct Table {
    pub rows: Vec<Vec<String>>,
    pub types: Vec<String>,
}

impl Table {
    pub fn new(headers: Vec<String>) -> Table {
        println!("{headers:?}");

        Table { rows: vec![headers.clone()],
            types: Vec::new(),
        }
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut result = "".to_string();
        self.rows.iter().for_each( |row| {
            row.iter().for_each( |value| {
                result.push_str(value.as_str());
                result.push_str(", ");
            });
            result.pop();
            result.pop();
            result.push_str("\n");
        });

        write!(f, "{result}")
    }
}
