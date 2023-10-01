use crate::compile::code::Code;

/// A value that can be taken as argument by a token
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
        // If the value starts with 'r', it's a register
        if value.starts_with('r') {
            // Skip the r and trim the rest
            let register = value.chars().skip(1).collect::<String>();
            let register = register.trim();

            // Try to parse it to a u8 as there are 256 registers
            // Panic on failure
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
            // If it isn't a register, it should be a number
            // Try to parse the number
            Self::Number(number)
        } else {
            // If it isn't a number either, it isn't a valid argument
            panic!(
                "Invalid argument \"{value}\": {}:{}",
                code.line(),
                code.column()
            );
        }
    }

    /// Take the value of the register, if ```self``` is a register
    /// Otherwise, ```self```
    pub fn take(&self, registers: &[Self]) -> Self {
        match self {
            Value::Register(register) => registers[*register as usize],
            value => *value,
        }
    }

    /// Tries to perform an addition operation
    pub fn add(&self, other: &Self, registers: &[Self]) -> Self {
        // Take the value of both values, so we only add the actual values
        let left = self.take(registers);
        let right = other.take(registers);

        match (left, right) {
            // If the values are numbers, add the numbers
            (Self::Number(number), Self::Number(number2)) => Self::Number(number + number2),

            // If any of the values is still a register, panic
            (Self::Register(_), _) | (_, Self::Register(_)) => {
                panic!("Unexpected register during addition");
            }
        }
    }

    /// Tries to perform an subtraction operation
    pub fn sub(&self, other: &Self, registers: &[Self]) -> Self {
        // Take the value of both values, so we only subtract the actual values
        let left = self.take(registers);
        let right = other.take(registers);

        match (left, right) {
            // If the values are numbers, subtract the numbers
            (Self::Number(number), Self::Number(number2)) => Self::Number(number - number2),

            // If any of the values is still a register, panic
            (Self::Register(_), _) | (_, Self::Register(_)) => {
                panic!("Unexpected register during addition");
            }
        }
    }

    /// Tries to perform an multiplication operation
    pub fn mul(&self, other: &Self, registers: &[Self]) -> Self {
        // Take the value of both values, so we only multiply the actual values
        let left = self.take(registers);
        let right = other.take(registers);

        match (left, right) {
            // If the values are numbers, multiply the numbers
            (Self::Number(number), Self::Number(number2)) => Self::Number(number * number2),

            // If any of the values is still a register, panic
            (Self::Register(_), _) | (_, Self::Register(_)) => {
                panic!("Unexpected register during addition");
            }
        }
    }
}
