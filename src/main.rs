use std::{error::Error, io::{self, BufRead}};
use regex::Regex;

static REGEXES: [&str; 5] = [
    //Get inline javascript variables defined with var, const or let
    r"(?:var|const|let)\s+([a-zA-Z_]\w*)\s*=",
    // Get JSON keys
    r#"["']{0,1}([a-zA-Z0-9$_\.-]*?)["']{0,1}:"#,
    // Get HTML input names
    r#"<input (?:[^>]*name=["']([^'"]*)|)"#,
    // Get HTML input ids
    r#"<input (?:[^>]*id=["']([^'"]+)|)"#,
    // Get paramter names
    r"[\?&](?:([^=]+)=)?"
];
// TODO: combile all regexes in just one &str varable
static JS_STRINGS: &str = r#"(?:|\[|,|, | ,)\s*['"]([a-zA-Z0-9\_-]+)['"]\s*(?:\s|\]|\)|,|, | ,)"#;


fn find_params(body: &String, printed: &mut Vec<String>, include_js_strings: bool) {
    let bad_regex = Regex::new(r#"["'\(\):&\[\] ;#]|(^[0-9]+$)"#).unwrap();
    let mut r: Vec<&str> = [&REGEXES[..]].concat();
    if include_js_strings {
        r = [&REGEXES[..], &[JS_STRINGS]].concat();
    }

    for re in r {

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
    let args: Vec<String> = std::env::args().collect();
    let include_js_strings = match args.get(1) {
        Some(val) => if val == "strings" {true} else {false},
        None => false
    };

    loop {

        let red_byte = stdin.read_line(&mut buffer)?;

        if red_byte == 0 {
            find_params(&body, &mut printed, include_js_strings);
            break;
        }

        count +=1;
        body += &buffer;

        if count == 10000 {
            find_params(&body, &mut printed, include_js_strings);
            count = 1;
            body.clear()
        }

        buffer.clear();
    }

    Ok(())
}
