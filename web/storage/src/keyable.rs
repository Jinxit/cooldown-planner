use std::fmt::Display;

pub trait Keyable {
    fn to_key(&self) -> String;
}

impl Keyable for &str {
    fn to_key(&self) -> String {
        self.to_string()
    }
}

impl Keyable for String {
    fn to_key(&self) -> String {
        self.clone()
    }
}

impl<T1: Display> Keyable for (T1,) {
    fn to_key(&self) -> String {
        self.0.to_string()
    }
}

impl<T1: Display, T2: Display> Keyable for (T1, T2) {
    fn to_key(&self) -> String {
        format!("{} {}", self.0, self.1)
    }
}

impl<T1: Display, T2: Display, T3: Display> Keyable for (T1, T2, T3) {
    fn to_key(&self) -> String {
        format!("{} {} {}", self.0, self.1, self.2)
    }
}

impl<T1: Display, T2: Display, T3: Display, T4: Display> Keyable for (T1, T2, T3, T4) {
    fn to_key(&self) -> String {
        format!("{} {} {} {}", self.0, self.1, self.2, self.3)
    }
}
