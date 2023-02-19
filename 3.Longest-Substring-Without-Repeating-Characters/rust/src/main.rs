fn main() {
    println!("Hello, world!");
    println!("Test 1");
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    println!("Test 2");
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    println!("Test 3");
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    println!("Test 4");
    assert_eq!(length_of_longest_substring("".to_string()), 0);
    println!("Test 5");
    assert_eq!(length_of_longest_substring(" ".to_string()), 1);
    println!("Test 6");
    assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
    println!("Test 7");
    assert_eq!(length_of_longest_substring("dvcvf".to_string()), 3);
    println!("Test 8");
    assert_eq!(length_of_longest_substring("aabaab!bb".to_string()), 3);
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().into_iter().collect();
    let input_len = chars.len() as usize;
    let mut end: usize = 0;
    let mut buffer: Vec<char> = vec![];
    let mut best_len = 0;

    if input_len == 0 {
        return 0;
    }

    loop {
        if !buffer.contains(&chars[end]) {
            // if does not exits in buffer
            buffer.push(chars[end]);
        } else {
            // if duplicated
            let buffer_len = buffer.len();
            buffer.push(chars[end]);
            best_len = std::cmp::max(best_len, buffer_len);
            let index_of_previous = buffer.iter().position(|o| o == &chars[end]).unwrap();
            buffer.drain(0..=index_of_previous);
        }

        end += 1;

        if end == input_len {
            break;
        }
    }

    std::cmp::max(best_len as i32, buffer.len() as i32)
}
