use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn central_tendency() {
    fn find_mean(v: &Vec<i32>) -> f64 {
        let mut mean = 0.0;
        for x in v {
            mean += *x as f64;
        }
        mean /= v.len() as f64;
        return mean;
    }
    fn find_median(mut v: Vec<i32>) -> i32 {
        for i in (1..v.len()).rev() {
            for j in 0..i {
                if v[j] > v[j + 1] {
                    v.swap(j, j + 1);
                }
            }
        }
        let mid = v.len() / 2;
        let median = v[mid];
        return median;
    }
    fn find_mode(map: &HashMap<i32, i32>) -> i32 {
        let (mut mode, mut max) = (0, 0);
        for (key, value) in map {
            if *value > max {
                mode = *key;
                max = *value;
            }
        }
        return mode;
    }
    fn get_input() -> (Vec<i32>, HashMap<i32, i32>) {
        let stdin = io::stdin();
        let mut v = Vec::new();
        let mut map: HashMap<i32, i32> = HashMap::new();
        println!("Enter number of numbers, followed by that many numbers one per line");
        let mut input_text = String::new();
        stdin
            .read_line(&mut input_text)
            .expect("could not read from stdin");
        let n: i32 = input_text.trim().parse().expect("invalid input");
        for _ in 0..n {
            input_text.clear();
            stdin
                .read_line(&mut input_text)
                .expect("could not read from stdin");
            let x = input_text.trim().parse().expect("enter a number");
            v.push(x);
            let count = map.entry(x).or_insert(0);
            *count += 1;
        }
        return (v, map);
    }
    let (v, map) = get_input();
    let mean = find_mean(&v);
    let median = find_median(v.clone());
    let mode = find_mode(&map);
    println!(
        "Mean is {} and the Median is {} and the Mode is {}",
        mean, median, mode
    );
}

fn pig_latin() {
    let (vowel_addend, cons_addend) = ("hay", "ay");

    let get_input = || -> String {
        let stdin = io::stdin();
        let line = stdin
            .lock()
            .lines()
            .next()
            .expect("no line")
            .expect("reading failed");

        return line;
    };

    let convert_word = |word: &str| -> String {
        let mut letters = word.chars();
        let first = match letters.next() {
            Some(letter) => letter,
            None => return String::new(),
        };
        let rest = letters.as_str();

        return match first.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-{}", word, vowel_addend),
            _ => format!("{}-{}{}", rest, first, cons_addend),
        };
    };

    println!("Enter sentence to be converted to pig latin");
    let s = get_input();
    let mut mapped = s.split_whitespace().map(convert_word);
    let mut result = String::new();

    if let Some(token) = mapped.next() {
        result.push_str(&token);
        for token in mapped {
            result.push(' ');
            result.push_str(&token);
        }
    }
    println!("The sentence in pig latin is {}", result);
}

fn emp_dept() {
    let add_employee = |map: &mut HashMap<String, Vec<String>>, name: &str, dept: &str| {
        map.entry(String::from(dept))
            .or_insert(Vec::new())
            .push(String::from(name));
            
        println!("Added employee {} in {}", name, dept);
    };

    let remove_employee = |map: &mut HashMap<String, Vec<String>>, name: &str, dept: &str| {
        if let Some(names) = map.get_mut(dept) {
            if names.contains(&String::from(name)){
                names.retain(|x| *x != name);
                println!("Removed employee {} from {}", name, dept);
            } else {
                println!("Employee {} doesn't exist in {}", name ,dept);   
            }
        }
        else {
            println!("Department {} doesn't exist yet", dept);
        }
    };

    let stdin = io::stdin();
    let mut map = HashMap::new();
    println!("Enter commands in the form 'Add/Remove name to/from department'");

    loop {
        let mut command = String::new();
        stdin
            .read_line(&mut command)
            .expect("could not read from stdin");
        let splitted: Vec<&str> = command.split_whitespace().take(4).collect();
        if let [command, name, _to_or_from_, dept] = splitted[..] {
            match &command.to_ascii_lowercase()[..] {
                "add" => add_employee(&mut map, name, dept),
                "remove" => remove_employee(&mut map, name, dept),
                _ => println!("Unsupported command, use add or remove"),
            };
        }
        else {
            println!("{:?}", map);
            break;
        }
    }
}

fn main() {
    central_tendency();
    pig_latin();
    emp_dept();
}
