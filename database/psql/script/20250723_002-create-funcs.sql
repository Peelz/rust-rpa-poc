-- sqlfluff:dialect:postgres


-- create_user_binding_tx 
DROP FUNCTION IF EXISTS create_user_binding_tx(
    p_session_id VARCHAR,
    p_binding_id INT,
    p_user_account_id INT,
    p_user_profile_id INT,
    p_policy_ref VARCHAR,
    p_binding_status INT DEFAULT 1
);

CREATE OR REPLACE FUNCTION create_user_binding_tx(
    p_session_id VARCHAR,
    p_binding_id INT,
    p_user_account_id INT,
    p_user_profile_id INT,
    p_policy_ref VARCHAR,
    p_binding_status INT DEFAULT 1
)
RETURNS INT AS $$
DECLARE
    v_tx_id INT;
BEGIN
    INSERT INTO user_binding_tx (
        session_id,
        binding_id,
        user_account_id,
        user_profile_id,
        policy_ref,
        binding_status
    )
    VALUES (
        p_session_id,
        p_binding_id,
        p_user_account_id,
        p_user_profile_id,
        p_policy_ref,
        p_binding_status
    )
    RETURNING tx_id INTO v_tx_id;

    RETURN v_tx_id;
END;
$$ LANGUAGE plpgsql;


-- insert_user_credit
DROP FUNCTION IF EXISTS insert_user_credit(
    p_tx_id INT,
    p_policy_active_at TIMESTAMPTZ,
    p_policy_inactive_at TIMESTAMPTZ,
    p_max_coverage_per_usage NUMERIC(18, 2),
    p_parent_credit_id INT DEFAULT NULL
);
CREATE OR REPLACE FUNCTION insert_user_credit(
    p_tx_id INT,
    p_policy_active_at TIMESTAMPTZ,
    p_policy_inactive_at TIMESTAMPTZ,
    p_max_coverage_per_usage NUMERIC(18, 2),
    p_parent_credit_id INT DEFAULT NULL
)
RETURNS INT AS $$
DECLARE
    v_credit_id INT;
BEGIN
    -- Step 1: Close existing active credit if any
    UPDATE user_credits
    SET credit_terminated_at = NOW()
    WHERE tx_id = p_tx_id
      AND credit_terminated_at IS NULL;

    -- Step 2: Insert new credit
    INSERT INTO user_credits (
        tx_id,
        policy_active_at,
        policy_inactive_at,
        max_coverage_per_usage,
        parent_credit_id
    )
    VALUES (
        p_tx_id,
        p_policy_active_at,
        p_policy_inactive_at,
        p_max_coverage_per_usage,
        p_parent_credit_id
    )
    RETURNING credit_id INTO v_credit_id;

    RETURN v_credit_id;
END;
$$ LANGUAGE plpgsql;
