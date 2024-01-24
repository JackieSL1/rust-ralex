use std::fmt::{Display, Formatter, Error};
use crate::{condition::Condition, parser::List};
use std::collections::HashMap;
use std::iter::zip;
use itertools::Itertools;

#[derive (Debug, Clone)]
pub struct Table {
    pub rows: Vec<Vec<String>>,
    pub types: Vec<String>,
}

impl Table {
    pub fn new(headers: Vec<String>) -> Table {
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
            return Err("error: tables must have same columns to minus");
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
        let attributes: List = self.rows[0].clone().into_iter().filter( |elem| !other.rows[0].contains(elem)).collect();
        let result = self.project(attributes.clone()).unwrap().multiply(other);
        let result = result.unwrap().minus(self);
        let result = result.unwrap().project(attributes.clone());
        let result = self.project(attributes).unwrap().minus(&result.unwrap());

        result
    }

    pub fn select(&self, condition: &Box<Condition>) -> Result<Table, &'static str> {
        let mut result = Table {
            rows: vec![self.rows[0].clone()],
            types: self.types.clone(),
        };

        for self_row in self.rows.iter().skip(1) {
            if Self::row_meets_condition(&self.rows[0], &self_row, &condition) {
                result.rows.push(self_row.clone());
            }
        }

        Ok(result)
    }

    fn row_meets_condition(header: &Vec<String>, row: &Vec<String>, condition: &Condition) -> bool {
        let mut row_lookup = HashMap::new();
        for (key, value) in zip(header.clone(), row.clone()) {
            row_lookup.insert(key, value); 
        }

        condition.eval(&row_lookup).parse().unwrap()
    }

    pub fn project(&self, columns: List) -> Result<Table, &'static str> {
        let mut result = self.clone();
        let mut columns = columns.clone();

        for column in self.rows[0].iter() {
            if !columns.contains(&column) { 
                result = result.remove_column(column);
            } else {
                if let Some(index) = columns.iter().position( |elem| elem == column) {
                    columns.remove(index);
                }
            }
        }

        result.rows = result.rows.into_iter().unique().collect();

        Ok(result)
    }

    fn remove_column(&self, column: &str) -> Table {
        let index = self.rows[0].iter().position( |col| col == column).unwrap();

        let mut result = self.clone();
        result.rows = result.rows
            .into_iter().map(|row| {
                row.into_iter()
                    .enumerate()
                    .filter( |&(i, _)| i != index)
                    .map( |(_, elem)| elem)
                    .collect::<Vec<String>>()
            })
            .collect();
        result.types.remove(index);

        result 
    }

    pub fn join(&self, condition: &Box<Condition>, other: &Table) -> Result<Table, &'static str> {
       self.multiply(other).unwrap().select(condition)
    }

    pub fn left_join(&self, condition: &Box<Condition>, other: &Table) -> Result<Table, &'static str> {
        let attributes: List = self.rows[0].clone().into_iter().collect();
        let mut null_row = Table {
            rows: vec![other.rows[0].clone().into_iter().filter( |elem| !self.rows[0].contains(elem)).collect()],
            types: other.types.clone() 
        };
        null_row.rows.push(null_row.rows[0].clone().into_iter().map( |_| "Null".to_string()).collect());
        self.join(condition, other).unwrap()
            .union(&self.minus(&self.join(condition, other).unwrap().project(attributes).unwrap()).unwrap()
            .multiply(&null_row).unwrap())
    }

    pub fn right_join(&self, condition: &Box<Condition>, other: &Table) -> Result<Table, &'static str> {
        let attributes: List = other.rows[0].clone().into_iter().collect();
        let mut null_row = Table {
            rows: vec![self.rows[0].clone().into_iter().filter( |elem| !other.rows[0].contains(elem)).collect()],
            types: self.types.clone() 
        };
        null_row.rows.push(null_row.rows[0].clone().into_iter().map( |_| "Null".to_string()).collect());
        self.join(condition, other).unwrap()
            .union(&null_row
            .multiply(&other.minus(&self.join(condition, other).unwrap().project(attributes).unwrap()).unwrap()).unwrap())
    }

    pub fn full_join(&self, condition: &Box<Condition>, other: &Table) -> Result<Table, &'static str> {
        self.left_join(condition, other).unwrap().union(&self.right_join(condition, other).unwrap())
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
