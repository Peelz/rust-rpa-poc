-- sqlfluff:dialect:postgres
--
-- id SERIAL PRIMARY KEY,
-- binding_id INT NOT NULL,
-- user_account_id INT NOT NULL,
-- user_profile_id INT NOT NULL,
-- insurance_policy_ref JSONB

CREATE OR REPLACE FUNCTION insert_policy(
    p_binding_id INT,
    p_user_account_id INT,
    p_user_profile_id INT,
    p_insurance_policy_ref JSONB
)
RETURNS VOID AS $$
BEGIN
    INSERT INTO user_insurance_policies (binding_id, user_account_id, user_profile_id, insurance_policy_ref)
    VALUES (p_binding_id, p_user_account_id, p_user_profile_id, p_insurance_policy_ref);
END;
$$ LANGUAGE plpgsql;
