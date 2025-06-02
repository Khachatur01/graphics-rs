use std::collections::HashMap;

pub struct SVGStyle(HashMap<&'static str, &'static str>);

impl SVGStyle {
    pub fn set(&mut self, style: &'static str, value: &'static str) {
        self.0.insert(style, value);
    }

    pub fn unset(&mut self, style: &'static str) {
        self.0.remove(style);
    }
}

impl Default for SVGStyle {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl Iterator for SVGStyle {
    type Item = (&'static str, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().next().map(|(k, v)| (*k, *v))
    }
}

impl<const N: usize> From<[(&'static str, &'static str); N]> for SVGStyle {
    fn from(value: [(&'static str, &'static str); N]) -> Self {
        Self(HashMap::from(value))
    }
}
