pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

// 1. Finish the implementation of LogQuery
impl<'a> LogQuery<'a> {
    // 2. Create a public associated function named `new()` that will take a reference to a vector of strings
    pub fn new(logs: &'a Vec<String>) -> LogQuery {
        Self {
            logs
        }
    }

    // 3. Create a public method named `search` that accepts a string slice and finds it from the logs and
    //    returns a vector of references to those logs.
    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        // This takes 3-5 ms, uses 88kB:
        self.logs.iter().filter(|record| {record.contains(keyword)}).collect::<Vec<&'a String>>()
        // This takes 3-4 ms, uses 88kB (slightly faster):
        // let mut results = Vec::new();
        // for record in self.logs.iter() {
        //     if record.contains(keyword) {
        //         results.push(record);
        //     }
        // }
        // results
    }
}

