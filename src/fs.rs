use std::io::Write;

pub fn append_to_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    // check if file exists
    let mut file = match std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
    {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::prelude::*;

    #[test]
    fn test_append_to_file() {
        // generate a random path
        let path = format!("/tmp/{}", rand::random::<u64>());
        let path = path.as_str();

        let content = "test";
        append_to_file(path, content).unwrap();
        let mut file = fs::File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, content);

        append_to_file(path, content).unwrap();
        let mut file = fs::File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, format!("{}{}", content, content));
    }
}
