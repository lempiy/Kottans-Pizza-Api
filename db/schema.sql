--person
DROP TABLE IF EXISTS person;
CREATE TABLE person (
  id serial primary key,
  uuid UUID unique not null,
  username TEXT unique null,
  email TEXT not null,
  password TEXT not null,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  last_login TIMESTAMP WITH TIME ZONE
);

CREATE UNIQUE INDEX uuid_idx ON person (uuid);
CREATE UNIQUE INDEX username_password_idx ON person (username, password);

INSERT INTO person(uuid, username, email, password, created_at)
  VALUES('d160fe6c-20a1-41d1-a331-2383d6a185ce', 'lempiy', 'lempiy@gmail.com',
        'q1w2e3r4', now());
