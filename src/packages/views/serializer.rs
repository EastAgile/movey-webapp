use convert_case::{Boundary, Case, Casing};
use serde::Serialize;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Eq)]
pub struct SerializableInvitation {
    pub status: Status,
    pub email: String,
}

impl PartialEq for SerializableInvitation {
    fn eq(&self, other: &SerializableInvitation) -> bool {
        self.email == other.email
    }
}

impl Hash for SerializableInvitation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}

#[derive(Serialize, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Status {
    Owner,
    PendingOwner,
    Collaborator,
    PendingCollaborator,
    External,
}

pub fn slugify_package_name(name: &str) -> String {
    slug::slugify(
        name.from_case(Case::Pascal)
            .without_boundaries(&[Boundary::UpperDigit])
            .to_case(Case::Kebab),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn slugify_package_name_works() {
        let original_name = [
            "ABIStruct",
            "AccountExample",
            "ERC20DecimalsMock",
            "DPNFramework",
            "aptETH",
            "DotAptTLD",
            "SVGNFT",
            "CounterCrossCall",
            "@EastAgile/test_move_package",
        ];
        let converted_name = original_name.map(|name| slugify_package_name(name));
        let expected_name = [
            "abi-struct",
            "account-example",
            "erc20-decimals-mock",
            "dpn-framework",
            "apt-eth",
            "dot-apt-tld",
            "svgnft",
            "counter-cross-call",
            "east-agile-test-move-package",
        ];
        for i in 0..9 {
            assert_eq!(
                converted_name[i], expected_name[i],
                "{} is not equal to {}",
                converted_name[i], expected_name[i]
            )
        }
    }
}
