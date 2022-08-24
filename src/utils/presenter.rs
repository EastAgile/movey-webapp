use jelly::prelude::*;
use jelly::DieselPgPool;
use jelly::Result;

use crate::accounts::Account;
use crate::packages::Package;

pub fn censor_email(email: &str) -> Result<String> {
    let mut censored_email = String::new();
    censored_email.push_str(&email[0..1]);
    censored_email.push_str("***");
    censored_email.push_str(
        &email[email
            .find('@')
            .ok_or_else(|| Error::Generic("Invalid email".to_string()))?..],
    );
    Ok(censored_email)
}

pub async fn make_account_name(package: &Package, db: &DieselPgPool) -> Result<String> {
    Ok(if let Some(uid) = package.account_id {
        let account = Account::get(uid, db).await?;
        if account.name.is_empty() {
            // If account doesn't have a name, it is a Github-only account
            if let Some(github_login) = account.github_login {
                github_login
            } else {
                account.email
            }
        } else {
            account.name
        }
    } else {
        // Default account name is derived from https://github.com/<github login>
        let repo_url = package.repository_url.clone();
        let derived_name = repo_url.split('/').collect::<Vec<&str>>()[3];
        derived_name.to_string()
    })
}

pub fn make_package_install_instruction(repo_url: &str) -> (String, String) {
    // Display url for install instruction
    // example: https://github.com/move-language/move/tree/main/language/evm/hardhat-examples/contracts/ABIStruct
    //          -> repo_url: https://github.com/move-language/move
    //             subdir: language/evm/hardhat-examples/contracts/ABIStruct
    let mut instruction_subdir = String::from("");
    let mut instruction_repo_url: String;
    let repo_url_tokens = repo_url.split('/').collect::<Vec<&str>>();
    if repo_url_tokens.len() > 7 {
        instruction_repo_url = repo_url_tokens[..5].join("/");
        instruction_subdir = repo_url_tokens[7..].join("/");
    } else {
        // Should be the root directory, not a subdir,
        // like https://github.com/move-language/move
        instruction_repo_url = repo_url.to_string();
    }
    instruction_repo_url.push_str(".git");
    (instruction_repo_url, instruction_subdir)
}

pub fn validate_name_and_version(package_name: &str, package_version: &str) -> Vec<&'static str> {
    let mut hints = vec![];
    let name_regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !name_regex.is_match(package_name) {
        hints.push(
            "Package name should only contain alphanumeric characters, hyphens or underscores",
        );
    }
    if semver::Version::parse(package_version).is_err() {
        hints.push("Package version should adhere to semantic versioning (see https://semver.org)");
    }
    hints
}

#[cfg(test)]

mod tests {
    use super::*;

    #[actix_rt::test]
    async fn validate_name_and_version_works_for_names() {
        let valid_package_names = vec![
            "MoveStdLib",
            "A_Certain_Package",
            "package103",
            "package-in-kebab-case",
            "101-dalmatians",
            "up-and_down__and--up",
            "-package-name-",
            "-_-_-zigzag-package-name-_-_-",
        ];
        for name in valid_package_names {
            let hints = validate_name_and_version(name, "0.1.0");
            assert!(hints.is_empty());
        }
        let invalid_package_names = vec![
            "special_package!",
            "package_歷要人",
            "@MystenLabs/Sui",
            "new.package.dot.com",
            "package-1.0.3",
            "invalid/package",
        ];
        for name in invalid_package_names {
            let hints = validate_name_and_version(name, "0.1.0");
            assert_eq!(hints.len(), 1);
            assert_eq!(
                hints[0],
                "Package name should only contain alphanumeric characters, hyphens or underscores"
            );
        }
    }

    #[actix_rt::test]
    async fn validate_name_and_version_works_for_versions() {
        let valid_versions = vec![
            "1.0.0-alpha",
            "1.0.0-alpha.1",
            "1.0.0-alpha.beta",
            "1.0.0-beta",
            "1.0.0-beta.2",
            "1.0.0-beta.11",
            "1.0.0-rc.1",
            "1.0.0",
        ];
        for version in valid_versions {
            let hints = validate_name_and_version("valid_name", version);
            assert!(hints.is_empty());
        }
        let invalid_versions = vec![
            "1.0.0-",
            "1.0.0-alpha+pre+release",
            "1.0.0*beta",
            "1.01.100",
            "1.0",
            "new.version",
        ];
        for version in invalid_versions {
            let hints = validate_name_and_version("valid_name", version);
            assert_eq!(hints.len(), 1, "version: {}", version);
            assert_eq!(
                hints[0],
                "Package version should adhere to semantic versioning (see https://semver.org)",
                "version: {}",
                version
            );
        }
    }
}
