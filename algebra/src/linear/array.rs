use serde::de::{DeserializeOwned, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Array<V, const N: usize> {
    values: [V; N]
}

impl<V, const N: usize> Array<V, N> {
    pub fn new(values: [V; N]) -> Self {
        Self { values }
    }
}

impl<V, const N: usize> Index<usize> for Array<V, N> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<V, const N: usize> IndexMut<usize> for Array<V, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<V: Copy + Default, const N: usize> Default for Array<V, N> {
    fn default() -> Self {
        Self {
            values: [V::default(); N]
        }
    }
}

impl<V: Copy + Default, const N: usize> From<[V; N]> for Array<V, N> {
    fn from(values: [V; N]) -> Self {
        Self { values }
    }
}

impl<V: Serialize, const N: usize> Serialize for Array<V, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        self.values.as_slice().serialize(serializer)
    }
}

impl<'de, V: DeserializeOwned + Default + Copy, const N: usize> Deserialize<'de> for Array<V, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Our custom Visitor will handle the actual deserialization logic
        deserializer.deserialize_seq(ArrayVisitor {
            marker: PhantomData, // For the generic type 'de
            array_marker: PhantomData, // For the const generic N
        })
    }
}

struct ArrayVisitor<V, const N: usize> {
    marker: PhantomData<fn() -> Array<V, N>>, // Helps the compiler with lifetimes
    array_marker: PhantomData<[(); N]>,     // Helps the compiler with const generics
}

impl<'de, V: DeserializeOwned + Default + Copy, const N: usize> Visitor<'de> for ArrayVisitor<V, N> {
    // The type that our Visitor produces
    type Value = Array<V, N>;

    // A human-readable message for what this Visitor expects
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!("an array of size {}", N))
    }

    // This method is called when the deserializer encounters a sequence (like a JSON array)
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>, // Provides methods to access elements in the sequence
    {
        // Create a mutable array initialized with default values (0.0 for f64)
        // This requires f64 to implement Default, which it does.
        let mut array = [V::default(); N];

        // Iterate through the elements in the sequence and populate our array
        for i in 0..N {
            // Get the next element from the sequence.
            // `next_element` returns an Option<T>, so we check if it's Some.
            if let Some(value) = seq.next_element()? {
                array[i] = value;
            } else {
                // If we run out of elements before filling the array, it's an error.
                return Err(serde::de::Error::invalid_length(
                    i, // The number of elements we actually got
                    &self, // The expected format from `expecting`
                ));
            }
        }

        // After filling the array, check if there are any extra elements in the input.
        // If there are, it's also an error because our fixed-size array can't hold them.
        if let Some(_) = seq.next_element::<V>()? {
            return Err(serde::de::Error::invalid_length(
                N + 1, // We got more than N elements
                &self,
            ));
        }

        // If everything went well, construct and return our Vector
        Ok(Array { values: array })
    }
}

