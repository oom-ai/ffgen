use crate::recipe::{RandGen, Recipe};
use serde::{Deserialize, Serialize};

pub type Schema = Entity;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename(serialize = "kebab-case"))]
#[serde(rename_all = "kebab-case")]
pub struct Entity {
    pub kind:        String,
    pub name:        String,
    pub description: Option<String>,
    pub groups:      Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Group {
    pub name:        String,
    pub category:    String,
    pub description: Option<String>,
    pub features:    Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Feature {
    pub name:        String,
    pub value_type:  String,
    pub description: Option<String>,
}

impl From<Recipe> for Entity {
    fn from(s: Recipe) -> Self {
        Entity {
            kind:        "Entity".into(),
            name:        s.entity.name,
            groups:      s
                .groups
                .into_iter()
                .map(|g| Group {
                    name:        g.name,
                    category:    "batch".into(),
                    features:    g
                        .features
                        .into_iter()
                        .map(|f| Feature {
                            name:        f.name,
                            value_type:  match f.rand_gen {
                                RandGen::Int { .. } => "int64".into(),
                                RandGen::Bool { .. } => "bool".into(),
                                RandGen::State => "string".into(),
                                RandGen::Enum { .. } => "string".into(),
                                RandGen::Timestamp { .. } => "int64".into(),
                                RandGen::Float { .. } => "float64".into(),
                                RandGen::DateTime { .. } => "time".into(),
                            },
                            description: f.description,
                        })
                        .collect(),
                    description: g.description,
                })
                .collect(),
            description: s.entity.description,
        }
    }
}
