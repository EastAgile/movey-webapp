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
