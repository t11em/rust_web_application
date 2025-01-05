INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Eleazar Fig',
    'eleazar.fig@example.com',
    '$2b$12$EJsVm9A/8mjAngQzauE54.bhSv9GjQcg1Co0cIQhy4jh5.uHlVUBW',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
