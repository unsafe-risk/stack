
pub fn pascal_to_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

pub fn snake_to_pascal(s: &str) -> String {
    let mut result = String::new();
    let mut upper = true;
    for c in s.chars() {
        if c == '_' {
            upper = true;
        } else {
            if upper {
                result.push(c.to_ascii_uppercase());
                upper = false;
            } else {
                result.push(c);
            }
        }
    }
    result
}

pub fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut upper = false;
    for c in s.chars() {
        if c == '_' {
            upper = true;
        } else {
            if upper {
                result.push(c.to_ascii_uppercase());
                upper = false;
            } else {
                result.push(c);
            }
        }
    }
    result
}

pub fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}
