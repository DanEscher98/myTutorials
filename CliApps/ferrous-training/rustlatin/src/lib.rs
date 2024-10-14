use url::Url;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn latinify(text: &str) -> String {
    text.split(' ')
        .map(|s| {
            let word = s.trim();
            let fst_char = word.chars().next().unwrap();
            if VOWELS.contains(&fst_char.to_ascii_lowercase()) {
                "sr".to_string() + word
            } else {
                word.to_string() + "rs"
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn url_matcher(data_file: &str) -> Result<(), std::io::Error> {
    let contents = std::fs::read_to_string(data_file)?;
    let num_lines = contents.lines().count();
    println!("From file with {}:", num_lines);

    for line in contents.lines() {
        if !line.is_empty() {
            match Url::parse(line) {
                Ok(url) => println!("Is a URL: {}", url),
                Err(_) => println!("Not a URL"),
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = latinify("i think there exists another way");
        assert_eq!(
            result,
            "sri thinkrs therers srexists sranother wayrs".to_string()
        );
    }

    #[test]
    fn test_url_matcher() {
        url_matcher("./data/contents.txt").unwrap();
    }
}
