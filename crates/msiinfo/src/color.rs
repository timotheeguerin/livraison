use std::io::IsTerminal;

pub fn formatter<T>(s1: &'static str, s2: &'static str) -> impl Fn(T) -> String
where
    T: AsRef<str>,
{
    move |t: T| format!("{}{}{}", s1, t.as_ref(), s2)
}

macro_rules! make_color {
    ( $( $name:ident: [$begin:expr, $end:expr]),* ) => {
        $(
           pub fn $name<T: AsRef<str>>(content: T) -> String {
               let content = content.as_ref();
               if std::io::stdout().is_terminal() {
                    format!("{}{}{}", $begin, content, $end)
                } else {
                    content.to_string()
                }
           }
        )*
    }
}

make_color! {
    reset: ["\x1b[0m", "\x1b[0m"],
    bold: ["\x1b[1m", "\x1b[22m"],
    dim: ["\x1b[2m", "\x1b[22m"],
    italic: ["\x1b[3m", "\x1b[23m"],
    underline: ["\x1b[4m", "\x1b[24m"],
    inverse: ["\x1b[7m", "\x1b[27m"],
    hidden: ["\x1b[8m", "\x1b[28m"],
    strikethrough: ["\x1b[9m", "\x1b[29m"],
    black: ["\x1b[30m", "\x1b[39m"],
    red: ["\x1b[31m", "\x1b[39m"],
    green: ["\x1b[32m", "\x1b[39m"],
    yellow: ["\x1b[33m", "\x1b[39m"],
    blue: ["\x1b[34m", "\x1b[39m"],
    magenta: ["\x1b[35m", "\x1b[39m"],
    cyan: ["\x1b[36m", "\x1b[39m"],
    white: ["\x1b[37m", "\x1b[39m"],
    gray: ["\x1b[90m", "\x1b[39m"],
    bg_black: ["\x1b[40m", "\x1b[49m"],
    bg_red: ["\x1b[41m", "\x1b[49m"],
    bg_green: ["\x1b[42m", "\x1b[49m"],
    bg_yellow: ["\x1b[43m", "\x1b[49m"],
    bg_blue: ["\x1b[44m", "\x1b[49m"],
    bg_magenta: ["\x1b[45m", "\x1b[49m"],
    bg_cyan: ["\x1b[46m", "\x1b[49m"],
    bg_white: ["\x1b[47m", "\x1b[49m"]
}
