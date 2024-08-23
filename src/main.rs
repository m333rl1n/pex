use regex::Regex;
use std::{
    error::Error,
    io::{self, BufRead}, process::exit,
};

static JAVASCRIPT_VARABLES: &str = r"(?:var|const|let)\s+([a-zA-Z_]\w*)\s*=";
static JSON_KEYS: &str = r#"["']{0,1}([a-zA-Z0-9$_\.-]*?)["']{0,1}:"#;
static HTML_INPUT_NAME: &str = r#"<input (?:[^>]*name=["']([^'"]*)|)"#;
static HTML_INPUT_ID: &str = r#"<input (?:[^>]*id=["']([^'"]+)|)"#;
static QUERY_STRING: &str = r"[\?&](?:([^=]+)=)?";
static FUNCTIONS_INPUTS: &str = r##".*\(\s*["|']?([\w\-]+)["|']?\s*(\,\s*["|']?([\w\-]+)["|']?\s*)?(\,\s*["|']?([\w\-]+)["|']?\s*)?(\,\s*["|']?([\w\-]+)["|']?\s*)?(\,\s*["|']?([\w\-]+)["|']?\s*)?(\,\s*["|']?([\w\-]+)["|']?\s*)?(\,\s*["|']?([\w\-]+)["|']?\s*)?(\,\s*["|']?([\w\-]+)["|']?\s*)?(\,\s*["|']?([\w\-]+)["|']?\s*)?(\,\s*["|']?([\w\-]+)["|']?\s*)?\)"##;
static JAVASCRIPT_STRINGS: &str = r#"(?:|\[|,|, | ,)\s*['"]([a-zA-Z0-9\_-]+)['"]\s*(?:\s|\]|\)|,|, | ,)"#;

fn find_params(body: &String, printed: &mut Vec<String>, s: bool) {
    let bad_regex = Regex::new(r#"["'\(\):&\[\] ;#]|(^[0-9]+$)"#).unwrap();
    let mut r: Vec<&str> = vec![
        JSON_KEYS,
        JAVASCRIPT_VARABLES,
        HTML_INPUT_ID,
        HTML_INPUT_NAME,
        QUERY_STRING,
        FUNCTIONS_INPUTS,
    ];
    if s {
        r.push(JAVASCRIPT_STRINGS)
    }

    for re in r {
        let re = Regex::new(re).unwrap();
        let result = re.captures_iter(&body);

        for m in result {
            let param = m.get(1).map_or("", |v| v.as_str()).trim();

            if !printed.contains(&param.to_string())
                && !param.is_empty()
                && (param.len() < 35)
                && !(bad_regex.is_match(&param))
            {
                println!("{}", param);
                printed.push(param.to_string());
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = io::stdin().lock();
    let mut buffer = String::new();
    let mut count = 0;
    let mut body = String::new();
    let mut printed: Vec<String> = vec![];
    let args: Vec<String> = std::env::args().collect();

    let s = if let Some(arg) = args.get(1) {
        if arg == "strings" {
            true
        } else {
            println!("Usage: pex [strings]");
            exit(1);
        }
    } else {
        false
    };

    loop {
        let red_byte = stdin.read_line(&mut buffer)?;

        if red_byte == 0 {
            find_params(&body, &mut printed, s);
            break;
        }

        count += 1;
        body += &buffer;

        if count == 10000 {
            find_params(&body, &mut printed, s);
            count = 1;
            body.clear()
        }

        buffer.clear();
    }

    Ok(())
}
