use std::io;

fn main() {
    let mut v_to_reverse = vec![];
    reverse_test(&mut v_to_reverse);
    dbg!(v_to_reverse);
    dbg!(word_to_pig_latin("apple"));
    dbg!(word_to_pig_latin("first"));
    dbg!(get_median_and_mode(&[1, 3, 5, 4, 1, 5, 5, 2, 2, 3]));
    test_mutation_on_hash_map();
    test_string_push();
    test_string_concat();
    test_match(Coin::Quarter(UsState::Alaska));
    let haystack = vec![String::from("a")];
    find_contains(&haystack[..], "b");
    test_copy();
    test_me2();
    add_big_strings(&mut vec!["a"], &["ab"]);
    let s = String::from("lok");
    test_me(&s);
    println!("Insert your number");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    println!("The number you inserted was: {num}");

    let num: u32 = num.trim().parse().expect("Please type a number!");

    let mut output = String::new();
    if num % 3 == 0 {
        output.push_str("Fizz");
    }
    if num % 5 == 0 {
        output.push_str("Buzz");
    }
    if output.len() == 0 {
        output.push_str(&num.to_string());
    }

    println!("{output}");
}

// fn round_in_place(v: &mut Vec<f32>) {
//     for n in v {
//         *n = n.round();
//     }
// }

// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
//     for s in src {
//         if s.len() > largest_len {
//             dst.push(*s);
//         }
//     }
// }

fn add_big_strings(dst: &mut Vec<&'static str>, src: &[&'static str]) {
    let largest: &str = dst.iter().max_by_key(|s| s.len()).unwrap();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s);
        }
    }
    println!("{dst:?}");
}

fn test_me(s: &String) -> Vec<&'static str> {
    let q = "a";
    let mut vq1 = vec![q];
    let vq2 = vec![q];
    let q0 = vq1[0];
    vq1.push(vq1[0]);
    vq1.push(vq2[0]);
    vq1.remove(0);
    println!("{vq1:?}");
    vq1
}

fn test_me2() {
    let mut a = [0, 1, 2, 3];
    let (a_l, a_r) = a.split_at_mut(2);
    let x = &mut a_l[1];
    let y = &a_r[0];
    *x += *y;
    println!("{a:?}");
}

fn test_copy() {
    let mut a: i32 = 0;
    let mut b: &mut i32 = &mut a;
    let c: &mut i32 = &mut b;
    *c += 1;
    *b += 1;
    println!("{b}");
    println!("{a}");
}

fn find_contains(haystack: &[String], needle: &str) {
    println!("haystack: {haystack:?} needle: {needle}");
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Texas,
    Colorado,
}

enum Coin {
    Quarter(UsState),
    Penny,
    Dime,
}

fn test_match(c: Coin) {
    match c {
        Coin::Penny => 1,
        Coin::Dime => 10,
        Coin::Quarter(UsState::Alaska) => {
            println!("Alaska!!!");
            25
        }
        Coin::Quarter(state) => {
            println!("state: {state:?}");
            25
        }
    };
}

fn test_string_concat() {
    let mut s1 = String::from("foo");
    let mut s2 = String::from("bar");
    let s3 = s1 + &s2;
    s2.push_str("a");
    println!("s2 is {s2}, s3 is {s3}");
}

fn test_string_push() {
    let mut s1 = String::from("foo");
    let mut s2 = String::from("bar");
    s1.push_str(&s2);
    s2.push_str("abe");
    println!("s2 is {s2}, s1 is {s1}");
}

fn test_mutation_on_hash_map() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }

    println!("{map:?}");
}

// Given a list of integers, use a vector and return the median (when sorted,
// the value in the middle position) and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.
fn get_median_and_mode(list: &[i32]) -> GetMedianAndModeResult {
    if list.len() == 0 {
        return GetMedianAndModeResult::ListIsEmpty;
    }

    let mut items = list.to_vec();
    items.sort();

    let mut mostFrequentElement = 0;
    let mut mostFrequentElementOccurrences = 0;

    let mut mostFrequentElementCandidate = items[0];
    let mut mostFrequentElementCandidateOccurrences = 0;

    let items_len = items.len();

    for (round, &i) in items.iter().enumerate() {
        if i == mostFrequentElementCandidate {
            mostFrequentElementCandidateOccurrences += 1;
        }
        if i != mostFrequentElementCandidate || round + 1 == items_len {
            if mostFrequentElementCandidateOccurrences > mostFrequentElementOccurrences {
                mostFrequentElement = mostFrequentElementCandidate;
                mostFrequentElementOccurrences = mostFrequentElementCandidateOccurrences;
            }
            mostFrequentElementCandidate = i;
            mostFrequentElementCandidateOccurrences = 1;
        }

        // dbg!(i);
        // dbg!(round);
        // dbg!(mostFrequentElementCandidate);
        // dbg!(mostFrequentElementCandidateOccurrences);

        // dbg!(mostFrequentElement);
        // dbg!(mostFrequentElementOccurrences);
    }

    return GetMedianAndModeResult::Result(MedianAndMode {
        median: items[items.len() / 2],
        mode: mostFrequentElement,
    });
}
#[derive(Debug)]
struct MedianAndMode {
    median: i32,
    mode: i32,
}
#[derive(Debug)]
enum GetMedianAndModeResult {
    ListIsEmpty,
    Result(MedianAndMode),
}

// Convert strings to pig latin. The first consonant of each word is moved to
// the end of the word and ay is added, so first becomes irst-fay. Words that
// start with a vowel have hay added to the end instead (apple becomes
// apple-hay). Keep in mind the details about UTF-8 encoding!
fn word_to_pig_latin(word: &str) -> String {
    if word.starts_with(&['a', 'e', 'i', 'o', 'u']) {
        word.to_owned() + &"-hay"
    } else {
        word[1..].to_string() + "-" + &word[0..0] + &"ay"
    }
}

fn reverse_test(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0..n / 2 {
        let p1 = &mut v[i] as *mut String;
        let p2 = &mut v[n - i - 1] as *mut String;
        unsafe {
            std::ptr::swap_nonoverlapping(p1, p2, 1);
        }
    }
}

fn mystery<T>(x: T) -> T {
    x
}

fn test_cond_lifetime_func() {
    let string1 = String::from("long string is long");
    let result;
    let a;
    let b;
    {
        let string2 = String::from("xyz");
        result = longest_cond_lifetime_struct(string1.as_str(), string2.as_str());
        AandB {a,b} = result;
    }
    println!("The longest string is {a:?}");
}

#[derive(Debug)]
enum AorB<A, B> {
    A(A),
    B(B),
}
fn longest_cond_lifetime_enum<'a, 'b>(a: &'a str, b: &'b str) -> AorB<&'a str, &'b str> {
    if a.len() > b.len() {
        AorB::A(a)
    } else {
        AorB::B(b)
    }
}

#[derive(Debug)]
struct AandB<A, B> {
    a: Option<A>,
    b: Option<B>,
}
fn longest_cond_lifetime_struct<'a, 'b>(a: &'a str, b: &'b str) -> AandB<&'a str, &'b str> {
    if a.len() > b.len() {
        AandB {
            a: Some(a),
            b: None,
        }
    } else {
        AandB {
            a: None,
            b: Some(b),
        }
    }
}

fn non_dangling_ref() -> &'static str {
    let result = "result";
    result
}
