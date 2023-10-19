use crate::value::Value;

#[derive(Clone)]
pub struct List {
    pub vector: Vec<Value>,
}

impl List {
    pub fn to_string(&self) -> String {
        format!(
            "[{}]",
            self.vector
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
