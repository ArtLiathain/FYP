pub mod map_vec_conversion {
    use serde::{Serializer, Deserializer};
    use std::collections::HashMap;
    use serde::ser::Serialize;
    use serde::de::Deserialize;

    pub fn serialize<S>(
        map: &HashMap<(usize, usize), usize>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let vec: Vec<((usize, usize), usize)> = map.iter().map(|(k, v)| (*k, *v)).collect();
        vec.serialize(serializer)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<(usize, usize), usize>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<((usize, usize), usize)> = Vec::deserialize(deserializer)?;
        Ok(vec.into_iter().collect())
    }
}
