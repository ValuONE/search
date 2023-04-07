#[cfg(test)]
mod tests {
    use std::fs::{read_to_string};

    use std::time::Instant;

    #[test]
    fn speed_test() {
        let query = "lights";

        let reader = read_to_string("./src/test/test.txt").unwrap();

        let contains_start = Instant::now();
        let _ = reader.contains(query);
        let duration = contains_start.elapsed();

        println!("Contains duration: {:#?}", duration);

        let mut results: Vec<String> = vec![];
        let contains_start = Instant::now();
        let mut line_count = 0;
        for line in reader.lines() {
            line_count += 1;
            if line.contains(query) {
                let _test = true;
                results.push(line.to_string());
            }
        }
        let duration_iteration = contains_start.elapsed();

        println!("Iteration duration: {:#?}", duration_iteration);

        assert_eq!(true, true);
    }
}