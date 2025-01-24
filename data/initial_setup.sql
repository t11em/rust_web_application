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
    '$2b$12$jKsdSjgEhgPwD6.g.INSienEEO3rQ2OHzaMC1r0ZvhGJ6M3tnbNa',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
