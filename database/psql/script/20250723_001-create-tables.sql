-- sqlfluff:dialect:postgres

CREATE TABLE enum_binding_status (
    status_id INT,
    status_title INT
);

CREATE TABLE user_binding_tx (
    tx_id SERIAL PRIMARY KEY,
    session_id VARCHAR NOT NULL,
    binding_id INT NOT NULL,
    user_account_id INT NOT NULL,
    user_profile_id INT NOT NULL,
    binding_status INT DEFAULT 1,
    policy_ref VARCHAR NOT NULL,
    created_at TIMESTAMPZ DEFAULT NOW()
);

CREATE TABLE user_credits (
    credit_id SERIAL PRIMARY KEY,
    tx_id INT NOT NULL,
    policy_active_at TIMESTAMPZ NOT NULL,
    policy_inactive_at TIMESTAMPZ NOT NULL,
    credit_terminated_at TIMESTAMPZ,
    max_coverage_per_usage NUMERIC(18, 2) NOT NULL,
    parent_credit_id INT,
    created_at TIMESTAMPZ DEFAULT NOW()
);

CREATE INDEX idx_events_ended_at_null ON events (id) WHERE ended_at IS NULL;
