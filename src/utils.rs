use svalue::SNumber;

pub fn find_numbers(text: &str) -> impl Iterator<Item = SNumber> {
    text.split(|c: char| !(c.is_ascii_digit() || c == '.' || c == ',' || c == '-' || c == '+'))
        .map(|x| x.trim_end_matches(['+', '-', ',', '.']))
        .filter(|x| !x.is_empty())
        .flat_map(to_number)
}

fn to_number(text: &str) -> Option<SNumber> {
    if let Some((prefix, suffix)) = text.split_once(['.', ','])
        && suffix.trim_end_matches('0').is_empty()
    {
        return prefix.parse().map(SNumber::Int).ok();
    }

    if let Ok(int) = text.parse() {
        Some(SNumber::Int(int))
    } else {
        text.replace(",", ".").parse().map(SNumber::Float).ok()
    }
}
