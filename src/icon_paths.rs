use std::sync::LazyLock;

pub struct IconPathData(LazyLock<Vec<&'static str>>);

impl IconPathData {
    pub const fn new(lazy_lock: LazyLock<Vec<&'static str>>) -> Self {
        Self(lazy_lock)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_icon_character(&self, icon_name_lower: &str) -> Option<char> {
        self.0
            .binary_search(&icon_name_lower)
            .ok()
            .and_then(|index| {
                let character_code = 0xe900u32 + (index as u32);
                char::from_u32(character_code)
            })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;
    use crate::icon_paths::IconPathData;

    const PATHS_DATA: &'static str = r#"suits
summits
sun
sun-cloud
sun-priest
sun-radiations
sun-spear
sunbeams
"#;

    #[test]
    pub fn test_get_icon() {
        let ipd = IconPathData::new(LazyLock::new(|| PATHS_DATA.lines().collect()));

        assert!(ipd.0.is_sorted());

        assert_eq!(ipd.get_icon_character("sun"), char::from_u32(0xe902));
    }
}
