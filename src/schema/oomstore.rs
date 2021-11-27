use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename(serialize = "kebab-case"))]
pub struct Entity {
    pub kind:           String,
    pub name:           String,
    pub length:         usize,
    pub batch_features: Vec<Group>,
    pub description:    Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename(serialize = "kebab-case"))]
pub struct Group {
    pub group:       String,
    pub features:    Vec<Feature>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename(serialize = "kebab-case"))]
pub struct Feature {
    pub name:          String,
    pub db_value_type: String,
    pub description:   Option<String>,
}

impl From<super::Schema> for Entity {
    fn from(s: super::Schema) -> Self {
        Entity {
            kind:           "Entity".into(),
            length:         s.entity.len(),
            name:           s.entity.name,
            batch_features: s
                .groups
                .into_iter()
                .map(|g| Group {
                    group:       g.name,
                    features:    g
                        .features
                        .into_iter()
                        .map(|f| Feature {
                            name:          f.name,
                            db_value_type: match f.rand_gen {
                                super::RandGen::Int { .. } => "int8".into(),
                                super::RandGen::Bool { .. } => "bool".into(),
                                super::RandGen::State => "text".into(),
                                super::RandGen::Enum { .. } => "text".into(),
                                super::RandGen::Timestamp { .. } => "int8".into(),
                            },
                            description:   f.description,
                        })
                        .collect(),
                    description: g.description,
                })
                .collect(),
            description:    s.entity.description,
        }
    }
}
