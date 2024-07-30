use std::collections::HashMap;

struct ListResult {
    median: f32,
    mode: i8,
}

/*
 * Given a list of integers, use a vector and return the median (when sorted, the
 * value in the middle position) and mode (the value that occurs most often; a hash map
 * will be helpful here) of the list.
 */
fn get_median_and_mode(list: &mut Vec<i8>) -> ListResult {
    list.sort();
    let i = list.len() / 2;
    let median = if list.len() % 2 == 0 {
        (list[i - 1] as f32 + list[i] as f32) / 2.0
    } else {
        list[i] as f32
    };

    let mut map = HashMap::new();

    let mut mode = 0;
    let mut max = 0;

    for &integer in list.iter() {
        let count = map.entry(integer).or_insert(0);
        *count += 1;
        if max < *count {
            max = *count;
            mode = integer;
        }
    }

    ListResult { median, mode }
}

// Convert strings to pig latin.
fn convert_str(text: &str) -> String {
    let vowels = "aeiouAEIOU";
    let mut result = String::new();
    for word in text.split_whitespace() {
        if let Some(first_char) = word.chars().next() {
            if vowels.contains(first_char) {
                result.push_str(&format!("{}-hay ", word));
            } else {
                if let Some(pos) = word.chars().position(|c| vowels.contains(c)) {
                    let (first, rest) = word.split_at(pos);
                    result.push_str(&format!("{}-{}ay ", rest, first));
                } else {
                    result.push_str(&format!("{}-ay ", word));
                }
            }
        }
    }

    result.trim_end().to_string()
}

/*
 * Using a hash map and vectors, create a text interface to allow a user to add employee names to a
 * department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let
 * the user retrieve a list of all people in a department or all people in the company by department,
 * sorted alphabetically.
 */

type Company = HashMap<String, Vec<String>>;

fn get_department<'a>(company: &'a Company, department: &'a str) -> Option<&'a Vec<String>> {
    company.get(department)
}

fn add_employee_to_department(company: &mut Company, command: &str) {
    let mut count = 0;
    let mut name = "";
    let mut department = "";
    for word in command.split_whitespace() {
        match count {
            0 => {
                if word != "Add" {
                    println!("wrong command");
                }
            }
            1 => name = word,

            2 => {
                if word != "to" {
                    println!("wrong command");
                }
            }
            3 => department = word,
            _ => {}
        }
        count += 1;
    }
    if let Some(d) = company.get_mut(department) {
        d.push(name.to_string());
        d.sort();
    } else {
        company.insert(department.to_string(), vec![name.to_string()]);
    }
}

fn main() {
    let mut list = vec![3, 1, 5, 7, 2, 8, 4, 1, 0];
    let ListResult { median, mode } = get_median_and_mode(&mut list);
    println!("Median is {median}, Mode is {mode}");

    let pig_latin = convert_str("hello apple Chichi");
    println!("{:?}", pig_latin);

    let mut company: Company = HashMap::new();

    add_employee_to_department(&mut company, "Add Sally to Engineering");
    add_employee_to_department(&mut company, "Add Goblin to Engineering");
    add_employee_to_department(&mut company, "Add Amir to Sales");
    if let Some(names) = get_department(&company, "Engineering") {
        println!("{:?}", names);
    };
}
