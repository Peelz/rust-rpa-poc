#[derive(Debug)]
pub enum RequestBinding {
    Group(GroupPolicyRequestBinding),
    Individual(IndividualPolicyRequestBinding),
}

#[derive(Debug)]
pub struct GroupPolicyRequestBinding {
    pub policy_holder_ref: String,
    pub insurred_member: String,
}

impl From<GroupPolicyRequestBinding> for RequestBinding {
    fn from(group: GroupPolicyRequestBinding) -> Self {
        RequestBinding::Group(group)
    }
}

#[derive(Debug)]
pub struct IndividualPolicyRequestBinding {
    policy_holder_ref: String,
    insurred_member: String,
}
