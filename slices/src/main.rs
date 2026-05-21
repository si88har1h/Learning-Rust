//Text Analyzer

fn main() {
    let s : String = String::from("the quick browned fox jumped");
    let part = &s[..];
    let first_word = first_word(part);
    let last_word = last_word(part);
    let word_count = word_count(part);


    println!("{}",first_word);
    println!("{}",last_word);
    println!("{}",word_count);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut last_space = 0;
    let mut is_space = false;

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            is_space = true;
            last_space = i;
        }
    }
    if !is_space {
        return &s[..];
    }
    &s[last_space+1 ..]
}    

fn word_count(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut count = 0;
    let mut space = 0;

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            count+=1;
            space=i;
        }
    }
    if s[space+1..].len() > 0 {
        count+=1;
    }

    count
}  

