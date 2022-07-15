use once_cell::sync::Lazy;
use crate::github_service::GithubRepoInfo;

pub const SHA_STUB: &str = "abcdef123456";

pub static GH_REPO_INFO_STUB: Lazy<GithubRepoInfo> = Lazy::new(|| {
    GithubRepoInfo {
        description: Some("description".to_string()),
        size: 10,
        default_branch: "".to_string()
    }
});
