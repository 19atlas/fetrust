pub mod yardimlar {
    use std::io::BufRead;

    pub fn read_lines<P: ?Sized>(file: &P) -> std::io::Result<
        std::io::Lines<std::io::BufReader<std::fs::File>>
    > where P: AsRef<std::path::Path>, {
        Ok(std::io::BufReader::new(
            std::fs::File::open(file)?).lines())
    }
}