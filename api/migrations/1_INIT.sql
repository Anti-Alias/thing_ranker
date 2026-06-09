-- Account roles
CREATE TYPE role AS ENUM ('basic', 'admin', 'root');


-- Accounts
CREATE TABLE account (
  id serial PRIMARY KEY,
  role role NOT NULL,
  email varchar(128) NOT NULL UNIQUE,
  name varchar(128) NOT NULL UNIQUE,
  password text NOT NULL,
  created timestamptz NOT NULL DEFAULT NOW(),
  modified timestamptz 
);
CREATE INDEX account_role_idx ON account(role);
CREATE INDEX account_created_idx ON account(created);
CREATE INDEX account_modified_idx ON account(modified);


-- Things
CREATE TABLE thing (
  id serial PRIMARY KEY,
  account_id integer NOT NULL REFERENCES account(id),
  name varchar(128) NOT NULL UNIQUE,
  image_name varchar(128),
  created timestamptz NOT NULL DEFAULT NOW(),
  modified timestamptz 
);
CREATE INDEX thing_created_idx ON thing(created);
CREATE INDEX thing_modified_idx ON thing(modified);


-- Categories
CREATE TABLE category (
  id serial PRIMARY KEY,
  account_id integer NOT NULL REFERENCES account(id),
  name varchar(128) NOT NULL UNIQUE,
  image_name varchar(128),
  created timestamptz NOT NULL DEFAULT NOW(),
  modified timestamptz 
);
CREATE INDEX category_created_idx ON category(created);
CREATE INDEX category_modified_idx ON category(modified);


-- The rankings of things in categories
CREATE TABLE rank (
  thing_id integer NOT NULL REFERENCES thing(id),
  category_id integer NOT NULL REFERENCES thing(id),
  wins integer NOT NULL,
  losses integer NOT NULL,
  win_loss_ratio float8 NOT NULL,
  created timestamptz NOT NULL DEFAULT NOW(),
  UNIQUE (thing_id, category_id)
);
CREATE INDEX rank_thing_id_idx ON rank(thing_id);
CREATE INDEX rank_category_id_idx ON rank(category_id);
CREATE INDEX rank_win_loss_ratio_idx ON rank(win_loss_ratio);
CREATE INDEX rank_created_idx ON rank(created);


-- Individual votes, comparing two things in a category.
-- Used to derive rankings.
CREATE TABLE vote (
  winning_thing_id integer NOT NULL REFERENCES thing(id),
  losing_thing_id integer NOT NULL REFERENCES thing(id),
  category_id integer NOT NULL REFERENCES thing(id),
  created timestamptz NOT NULL DEFAULT NOW()
);
CREATE INDEX vote_created_idx ON vote(created);


-- Seeds initial data
INSERT INTO account (role, email, name, password) VALUES ('root', 'admin@admin.com', 'admin', 'fake_password');
INSERT INTO category (account_id, name) VALUES (
  (SELECT id FROM account WHERE role='root'),
  'Thing'
);

