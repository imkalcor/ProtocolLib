use binary_derive::Binary;

use super::VarString;

#[derive(Default, Debug, Binary)]
pub struct EducationSharedResourceURI {
    pub button_name: VarString,
    pub link_url: VarString,
}

#[derive(Default, Debug, Binary)]
pub struct EducationEditionalLinkSettings {
    pub url: VarString,
    pub display_name: VarString,
}
