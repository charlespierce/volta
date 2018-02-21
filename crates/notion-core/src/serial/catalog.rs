use super::super::catalog;

use std::string::ToString;
use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::default::Default;

use semver::{Version, SemVerError};

#[derive(Serialize, Deserialize)]
pub struct Catalog {
    #[serde(default)]
    node: NodeCatalog
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "node")]
pub struct NodeCatalog {
    activated: Option<String>,
    versions: Vec<String>
}

impl Default for NodeCatalog {
    fn default() -> Self {
        NodeCatalog {
            activated: None,
            versions: vec![]
        }
    }
}

impl Catalog {
    pub fn into_catalog(self) -> Result<catalog::Catalog, SemVerError> {
        Ok(catalog::Catalog {
            node: self.node.into_node_catalog()?
        })
    }
}

impl NodeCatalog {
    fn into_node_catalog(self) -> Result<catalog::NodeCatalog, SemVerError> {
        let activated = match self.activated {
            Some(v) => Some(Version::parse(&v[..])?),
            None => None
        };

        let versions: Result<Vec<Version>, SemVerError> = self.versions.into_iter().map(|s| {
            Ok(Version::parse(&s[..])?)
        }).collect();

        Ok(catalog::NodeCatalog {
            activated: activated,
            versions: BTreeSet::from_iter(versions?)
        })
    }
}

impl catalog::Catalog {

    pub fn to_serial(&self) -> Catalog {
        Catalog {
            node: self.node.to_serial()
        }
    }

}
impl catalog::NodeCatalog {
    fn to_serial(&self) -> NodeCatalog {
        NodeCatalog {
            activated: self.activated.clone().map(|v| v.to_string()),
            versions: self.versions.iter().map(|v| v.to_string()).collect()
        }
    }
}
