CREATE TABLE user_insurance_policies (
    id SERIAL PRIMARY KEY,
    binding_id INT NOT NULL,
    user_account_id INT NOT NULL,
    user_profile_id INT NOT NULL,
    insurance_policy_ref JSONB
);
