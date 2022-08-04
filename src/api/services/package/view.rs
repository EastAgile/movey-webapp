use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PackageBadgeRespond {
    package_name: String,
    total_download_counts: i32,
    latest_version: String,
    versions: Vec<PackageBadgeVersion>,
}

#[derive(Serialize, Deserialize)]
pub struct PackageBadgeVersion {
    version: String,
    download_counts: i32,
}

impl From<Vec<(String, i32, String, i32)>> for PackageBadgeRespond {
    fn from(tuples: Vec<(String, i32, String, i32)>) -> Self {
        let mut max_version: String = tuples[0].2.to_string();
        let mut shield_respond = PackageBadgeRespond {
            package_name: tuples[0].0.to_string(),
            total_download_counts: tuples[0].1,
            latest_version: "".to_string(),
            versions: vec![],
        };
        for record in tuples {
            let version = PackageBadgeVersion {
                version: record.2.to_string(),
                download_counts: record.3,
            };
            if record.2 > max_version {
                max_version = record.2.to_string()
            }
            shield_respond.versions.push(version)
        }
        shield_respond.latest_version = max_version;
        shield_respond
    }
}
