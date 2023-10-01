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
        if value.starts_with('r') {
            let register = value.chars().skip(1).collect::<String>();
            let register = register.trim();
            register.parse::<u8>().map_or_else(
                |_| {
                    panic!(
                        "Invalid register id \"{register}\": {}:{}",
                        code.line(),
                        code.column()
                    )
                },
                Self::Register,
            )
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

    pub fn take(&self, registers: &[Self]) -> Self {
        match self {
            Value::Register(register) => registers[*register as usize],
            value => *value,
        }
    }

    pub fn add(&self, other: &Self, registers: &[Self]) -> Self {
        let left = self.take(registers);
        let right = other.take(registers);
        match (left, right) {
            (Self::Number(number), Self::Number(number2)) => Self::Number(number + number2),
            (Self::Register(_), _) | (_, Self::Register(_)) => {
                panic!("Unexpected register during addition");
            }
        }
    }

    pub fn sub(&self, other: &Self, registers: &[Self]) -> Self {
        let left = self.take(registers);
        let right = other.take(registers);
        match (left, right) {
            (Self::Number(number), Self::Number(number2)) => Self::Number(number - number2),
            (Self::Register(_), _) | (_, Self::Register(_)) => {
                panic!("Unexpected register during addition");
            }
        }
    }
}
