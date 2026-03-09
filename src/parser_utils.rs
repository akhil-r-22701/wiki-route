pub fn extract_quoted(s: &str) -> Option<(String, &str)> {
    let mut result = String::new();
    let mut chars = s.char_indices();

    loop {
        match chars.next()? {
            (_, '\\') => match chars.next()?.1 {
                '\'' => result.push('\''),
                '\\' => result.push('\\'),
                c => {
                    result.push('\\');
                    result.push(c);
                }
            },
            (i, '\'') => return Some((result, &s[i + 1..])),
            (_, c) => result.push(c),
        }
    }
}
