-- Add migration script here
CREATE TABLE subs(
	id uuid NOT NULL,
	PRIMARY KEY (id),
	email TEXT NOT NULL UNIQUE,
	name TEXT NOT NULL,
	subs_at timestamptz NOT NULL
);
