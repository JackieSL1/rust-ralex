#[derive (Debug)]
pub struct Table {
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: Vec<String>) -> Table {
        println!("{headers:?}");

        Table { rows: vec![headers.clone()] }
    }
   
    // pub fn fmt(&self, f: &mut fmt:Formatter) -> fmt::Result {
    //     
    //
    // }
}
