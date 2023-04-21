use std::process::exit;

pub fn checkformat(pattern: String) -> String
{
    let mut pat = pattern.clone();
    pat.make_ascii_lowercase();
    let pformat =  pat.to_string();
    let mut format: Vec<&str> = pformat.split(".").collect();
    format = format.into_iter().filter(|&i| i.trim() != "").collect::<Vec<_>>();
    let format = format[format.len()-1];
    let formats = vec!["eps","pdf","png","svg"]; 
    if formats.contains(&format) {return format.to_string();}
    else { println!("% Error! Wrong save format!"); exit(1); }
}