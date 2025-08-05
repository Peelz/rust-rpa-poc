-- sqlfluff:dialect:postgres

INSERT INTO enum_binding_status (status_id, status_title)
VALUES
(1, 'initiated'),
(2, 'rejected'),
(3, 'approved')
ON CONFLICT (status_id) DO UPDATE
    SET status_title = excluded.status_title;
