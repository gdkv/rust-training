use std::io::Error;

fn next_lang<'a>(languages: &'a [String], current_lang: &str) -> &'a str {
    let mut is_found: bool = false;

    for lang in languages {
        if is_found {
            return lang.as_str();
        }

        if lang == current_lang {
            is_found = true;
        }
    }

    last_lang(languages)
}

fn longest_lang<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a == lang_b {
        return lang_a;
    }

    if lang_a.len() > lang_b.len() { lang_a } else { lang_b }
}

fn last_lang(languages: &[String]) -> &str {
    languages.last().unwrap()
}
fn main() -> Result<(), Error> {
    let langs: Vec<String> = vec![
        String::from("en-US"),
        String::from("en-GB"),
        String::from("fr-FR"),
        String::from("ru-RU"),
        String::from("de-DE"),
        String::from("it-IT"),
        String::from("ja-JP"),
        String::from("sr-SJ"),
    ];

    let lang = next_lang(&langs, "it-IT");
    println!("{}", lang);

    let longest_lang = longest_lang(&langs[0], &langs[1]);
    println!("{}", longest_lang);

    Ok(())
}