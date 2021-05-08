use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};

use anyhow::Result;

type ChoiceCache = HashMap<String, Option<bool>>;

fn lookup_or_prompt(cache: &mut ChoiceCache, line: &str) -> Result<Option<bool>> {
    if let Some(b) = cache.get(line) {
        return Ok(*b);
    }

    let choice = loop {
        eprint!("{} [yni]? ", line);
        let mut ans = String::new();
        if io::stdin().read_line(&mut ans)? == 0 {
            break None;
        }
        if ans.starts_with('y') {
            break Some(true);
        } else if ans.starts_with('n') {
            break Some(false);
        } else if ans.starts_with('i') {
            break None;
        }
    };

    cache.insert(line.into(), choice);
    Ok(choice)
}

fn preproc(
    input: &mut dyn io::BufRead,
    allow: &mut dyn FnMut(&str) -> Result<Option<bool>>,
) -> Result<String> {
    let is_cond_start =
        |s: &str| s.starts_with("#if") || s.starts_with("#ifdef") || s.starts_with("#ifndef");
    let mut choice_stack = Vec::<Option<bool>>::new();
    let mut output = String::new();

    while let Some(line) = input.lines().next() {
        let line = line?;
        if is_cond_start(&line) {
            let choice = if choice_stack.contains(&Some(false)) {
                Some(false)
            } else {
                allow(&line)?
            };
            choice_stack.push(choice);
            if choice != None {
                // if we decided, don't print preprocessor line
                continue;
            }
        } else if line.starts_with("#else") {
            let choice = choice_stack.last_mut().unwrap();
            *choice = choice.map(|b| !b);
            if *choice != None {
                continue;
            }
        } else if line.starts_with("#endif") {
            let choice = choice_stack.pop().unwrap();
            if choice != None {
                continue;
            }
        }
        if !choice_stack.contains(&Some(false)) {
            output += &line;
            output.push('\n');
        }
    }

    Ok(output)
}

fn main() -> Result<()> {
    let mut cache = ChoiceCache::new();

    for arg in std::env::args_os().skip(1) {
        let output = preproc(
            &mut io::BufReader::new(File::open(&arg)?),
            &mut |s| lookup_or_prompt(&mut cache, s),
        )?;
        File::create(&arg)?.write_all(output.as_bytes())?;
    }

    Ok(())
}
