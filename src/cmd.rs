#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandKind {
    Sh, JbSh, JbJs,
    // TODO: sessions?
}

#[derive(Debug, Clone)]
pub struct Command<'a> {
    pub kind: CommandKind,
    pub code: &'a str,
}

fn parse_code(input: &str) -> &str {
    let input = input.trim();

    if input.starts_with("```") && input.ends_with("```") {
        let mut code = &input[3..input.len() - 3];
        if let Some(i) = code.find('\n') {
            code = &code[i + 1..];
        }
        return code.trim_suffix('\n');
    }

    if input.starts_with("`") && input.ends_with("`") {
        return input[1..input.len() - 1].trim();
    }

    input
}

pub fn parse<'a>(rest: &'a str) -> Option<Command<'a>> {
    let rest = rest.trim_start();

    let (kind, remainder) = if let Some(rem) = rest.strip_prefix("jb sh") {
        (CommandKind::JbSh, rem)
    } else if let Some(rem) = rest.strip_prefix("jb js") {
        (CommandKind::JbJs, rem)
    } else if let Some(rem) = rest.strip_prefix("sh") {
        (CommandKind::Sh, rem)
    } else {
        return None;
    };

    if !remainder.is_empty() && !remainder.starts_with(char::is_whitespace) {
        return None;
    }

    let code = parse_code(remainder);
    Some(Command { kind, code })
}
