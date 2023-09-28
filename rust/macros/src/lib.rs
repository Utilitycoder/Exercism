use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    () => {
        HashMap::new()
    };
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
}

#[macro_export]
macro_rules! myName {
    ($name:expr) => {
        "my name is ".to_string() + $name
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_myname() {
        assert_eq!(myName!("rust"), "my name is rust");
    }
}
