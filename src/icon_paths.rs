use std::{collections::BTreeMap, sync::Arc};
use ustr::Ustr;

#[derive(Debug, Clone, PartialEq)]
pub struct IconRow {
    pub name: String,
    pub path: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct IconPaths {
    pub map: Arc<BTreeMap<String, Ustr>>,
}

impl IconPaths {
    pub fn from_rows(data: &str) -> Result<Self, anyhow::Error> {
        let mut icons = vec![];
        for line in data.lines().map(|x| x.trim()).filter(|x| !x.is_empty()) {
            match line.split_once('\t') {
                Some((name, path)) => {
                    icons.push(IconRow {
                        name: name.to_string(),
                        path: path.to_string(),
                    });
                }
                None => anyhow::bail!("Could not split line '{line}'"),
            }
        }

        let map = icons
            .into_iter()
            .map(|x| (x.name, Ustr::from(x.path.as_str())))
            .collect();

        Ok(IconPaths { map: Arc::new(map) })
    }
}
