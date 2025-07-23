CREATE OR REPLACE FUNCTION insert_user_insurance_policy(
    p_user_id INT,
    p_insurance_policy_ref JSONB
)
RETURNS VOID AS $$
BEGIN
    INSERT INTO user_insurance_policies (user_id, insurance_policy_ref)
    VALUES (p_user_id, p_insurance_policy_ref);
END;
$$ LANGUAGE plpgsql;

