use std::collections::HashMap;

pub struct PropertyMap(HashMap<&'static str, &'static str>);

impl PropertyMap {
    pub fn set(&mut self, style: &'static str, value: &'static str) {
        self.0.insert(style, value);
    }

    pub fn unset(&mut self, style: &'static str) {
        self.0.remove(style);
    }
}

impl Default for PropertyMap {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl Iterator for &PropertyMap {
    type Item = (&'static str, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().next().map(|(k, v)| (*k, *v))
    }
}

impl<const N: usize> From<[(&'static str, &'static str); N]> for PropertyMap {
    fn from(value: [(&'static str, &'static str); N]) -> Self {
        Self(HashMap::from(value))
    }
}
