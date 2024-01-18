use std::{fmt::{Display, Formatter, Error}, collections::hash_map};
use crate::{parser::Expr, condition::Condition};
use std::collections::HashMap;
use std::iter::zip;

#[derive (Debug, Clone)]
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

    pub fn union(&self, other: &Table) -> Result<Table, &'static str> {
        if self.rows[0] != other.rows[0] {
           return Err("error: tables must have same columns to union");
        }

        let mut result = self.clone();
        for row in other.rows.iter().skip(1) {
            if !result.rows.contains(row) {
                result.rows.push(row.clone());
            }
        }
        Ok(result)
    }

    pub fn minus(&self, other: &Table) -> Result<Table, &'static str> {
        if self.rows[0] != other.rows[0] {
           return Err("error: tables must have same columns to union");
        }

        let mut result = Table {
            rows: vec![self.rows[0].clone()],
            types: self.types.clone(),
        };

        for row in self.rows.iter().skip(1) {
            if !other.rows.contains(row) {
                result.rows.push(row.clone());
            }
        }
        Ok(result)
    }

    pub fn intersect(&self, other: &Table) -> Result<Table, &'static str> {
        if self.rows[0] != other.rows[0] {
           return Err("error: tables must have same columns to intersect");
        }

        let mut result = Table {
            rows: vec![self.rows[0].clone()],
            types: self.types.clone(),
        };

        for self_row in self.rows.iter().skip(1) {
            for other_row in other.rows.iter().skip(1) {
                if self_row == other_row {
                    result.rows.push(self_row.clone());
                }
            }
        }

        Ok(result)
    }

    pub fn multiply(&self, other: &Table) -> Result<Table, &'static str> {
        let mut result = Table {
            rows: vec![self.rows[0].clone()],
            types: self.types.clone(),
        };
        result.types.extend(other.types.clone());
        result.rows[0].extend(other.rows[0].clone());

        for self_row in self.rows.iter().skip(1) {
            for other_row in other.rows.iter().skip(1) {
                let mut new_row = self_row.clone();
                new_row.extend(other_row.clone());
                result.rows.push(new_row);
            }
        }

        Ok(result)
    }

    pub fn divide(&self, other: &Table) -> Result<Table, &'static str> {
        todo!();
        let mut result = Table {
            rows: vec![self.rows[0].clone()],
            types: self.types.clone(),
        };
        result.types.extend(other.types.clone());
        result.rows[0].extend(other.rows[0].clone());

        for self_row in self.rows.iter().skip(1) {
            for other_row in other.rows.iter().skip(1) {
                let mut new_row = self_row.clone();
                new_row.extend(other_row.clone());
                result.rows.push(new_row);
            }
        }

        Ok(result)
    }

    pub fn select(&self, condition: &Box<Condition>) -> Result<Table, &'static str> {
        let mut result = Table {
            rows: vec![self.rows[0].clone()],
            types: self.types.clone(),
        };

        for self_row in self.rows.iter().skip(1) {
            let mut row_lookup = HashMap::new();
            for (key, value) in zip(self.rows[0].clone(), self_row.clone()) {
               row_lookup.insert(key, value); 
            }
            if condition.eval(&row_lookup).parse().unwrap() {
                result.rows.push(self_row.clone());
            }
        }

        Ok(result)
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
