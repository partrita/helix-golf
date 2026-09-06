use std::path::Path;
use std::{env, io};

use miette::miette;
use serde_json::Value;

pub fn mdbook_preprocessor() -> miette::Result<()> {
    match env::args().nth(2).as_deref() {
        Some("supports") => return Ok(()),
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    let input: (Value, Value) = serde_json::from_reader(io::stdin())
        .map_err(|err| miette!("failed to parse mdbook input: {err}"))?;

    let mut book = input.1;

    if let Some(items) = book.get_mut("items").and_then(Value::as_array_mut) {
        process_items(items);
    } else if let Some(sections) = book.get_mut("sections").and_then(Value::as_array_mut) {
        process_items(sections);
    }

    serde_json::to_writer(io::stdout(), &book)
        .map_err(|err| miette!("failed to write modified mdbook: {err}"))
}

fn process_items(items: &mut [Value]) {
    for item in items {
        if let Some(chapter) = item.get_mut("Chapter") {
            if let Some(content) = chapter
                .get("content")
                .and_then(Value::as_str)
                .map(str::to_string)
            {
                let path_stem = chapter
                    .get("path")
                    .and_then(Value::as_str)
                    .and_then(|p| Path::new(p).file_stem()?.to_str().map(str::to_string));

                if let (Some(name), Some(start)) = (path_stem, content.find("## Command"))
                    && name != "introduction"
                {
                    let (before, after) = content.split_at(start);
                    let new_content = format!(
                        "{before}\n## Preview\n\n<video controls>\n  <source src=\"generated/{name}.mp4\" type=\"video/mp4\">\n</video>\n\n{after}"
                    );
                    chapter["content"] = Value::String(new_content);
                }
            }

            if let Some(sub_items) = chapter.get_mut("sub_items").and_then(Value::as_array_mut) {
                process_items(sub_items);
            }
        }
    }
}
