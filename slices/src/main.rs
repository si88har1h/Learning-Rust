//Text Analyzer

fn main() {
    let s : String = String::from("the quick brown fox jumped");
    let part = &s[..];
    let first_word = first_word(part);
    let last_word = last_word(part);
    let longest_word = longest_word(part);
    let word_count = word_count(part);


    println!("First Word: {}",first_word);
    println!("Last Word: {}",last_word);
    println!("Longest Word: {}",longest_word);
    println!("Word Count: {}",word_count);
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

fn longest_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut space = 0;
    let mut longest_slice = "";

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' '{
            if s[space..i].trim().len() > longest_slice.len() {
                longest_slice = s[space..i].trim();
            }
            space = i;
        }
    }

    if s[space..].trim().len() > longest_slice.len(){
        longest_slice = s[space..].trim();
    }
    
    longest_slice
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

