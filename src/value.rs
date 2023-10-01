use crate::compile::code::Code;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Number(i64),
    Register(u8),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(value) => write!(f, "{value}"),
            Self::Register(value) => write!(f, "{value}"),
        }
    }
}

impl Value {
    pub fn from_str(value: &str, code: &Code) -> Self {
        if value.chars().next().is_some_and(|c| c == 'r') {
            let register = value.chars().skip(1).collect::<String>();
            let register = register.trim();
            if let Ok(register) = register.parse::<u8>() {
                Self::Register(register)
            } else {
                panic!(
                    "Invalid register id \"{register}\": {}:{}",
                    code.line(),
                    code.column()
                );
            }
        } else if let Ok(number) = value.parse::<i64>() {
            Self::Number(number)
        } else {
            panic!(
                "Invalid argument \"{value}\": {}:{}",
                code.line(),
                code.column()
            );
        }
    }
}
