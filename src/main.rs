use std::{error::Error, io::{self, BufRead}};
use regex::Regex;

static REGEXES: [&str; 5] = [
    //Get inline javascript variables defined with var, const or let
    r"(?:var|const|let)\s+([a-zA-Z_]\w*)\s*=",
    // Get JSON keys
    r#""([a-zA-Z0-9$_\.-]*?)":"#,
    // Get HTML input names
    r#"<input (?:[^>]*name=["']([^'"]*)|)"#,
    // Get HTML input ids
    r#"<input (?:[^>]*id=["']([^'"]+)|)"#,
    // Get paramter names
    r"[\?&](?:([^=]+)=)?"
];



fn find_params(body: &String, printed: &mut Vec<String>) {
    let bad_regex = Regex::new(r#"["'\(\):&\[\] ;#]"#).unwrap();

    for re in REGEXES {

        let re = Regex::new(re).unwrap();
        let result = re.captures_iter(&body);

        for m in result{
            let param = m.get(1).map_or("", |v| v.as_str()).trim();

            if !printed.contains(&param.to_string()) && !param.is_empty() && (param.len() < 35) && !(bad_regex.is_match(&param)){
                println!("{}", param);
                printed.push(param.to_string());
            }
        }

    }
}


fn main() -> Result<(), Box<dyn Error>>{
    let mut stdin = io::stdin().lock();
    let mut buffer = String::new();
    let mut count = 0;
    let mut body = String::new();
    let mut printed: Vec<String> = vec![];


    loop {

        let red_byte = stdin.read_line(&mut buffer)?;

        if red_byte == 0 {
            find_params(&body, &mut printed);
            break;
        }

        count +=1;
        body += &buffer;

        if count == 10000 {
            find_params(&body, &mut printed);
            count = 1;
            body.clear()
        }

        buffer.clear();
    }

    Ok(())
}
