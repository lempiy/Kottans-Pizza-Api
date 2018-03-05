BEGIN;

--counter trigger
DROP TABLE IF EXISTS rowcount;
CREATE TABLE rowcount (
    table_name  text NOT NULL,
    total_rows  bigint,
    PRIMARY KEY (table_name));

CREATE OR REPLACE FUNCTION count_rows()
	RETURNS TRIGGER AS $count_rows$
	   BEGIN
	      IF (TG_OP = 'INSERT') THEN
	         UPDATE rowcount
	            SET total_rows = total_rows + 1
	            WHERE table_name = TG_RELNAME;
	      ELSIF (TG_OP = 'DELETE') THEN
	         UPDATE rowcount
	            SET total_rows = total_rows - 1
	            WHERE table_name = TG_RELNAME;
	      END IF;
	      RETURN NULL;
	   END;
    $count_rows$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_count(text) RETURNS bigint
    AS $$ SELECT total_rows FROM rowcount WHERE table_name=$1 $$
    LANGUAGE SQL;

--person
DROP TABLE IF EXISTS person;
CREATE TABLE person (
  id SERIAL primary key,
  uuid UUID unique not null,
  username varchar(100) unique not null,
  email varchar(100) not null,
  password varchar(100) not null,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  last_login TIMESTAMP WITH TIME ZONE
);

CREATE UNIQUE INDEX uuid_idx ON person (uuid);
CREATE UNIQUE INDEX username_password_idx ON person (username, password);

CREATE TRIGGER countrows
  AFTER INSERT OR DELETE on person
  FOR EACH ROW EXECUTE PROCEDURE count_rows();

DELETE FROM rowcount WHERE table_name = 'person';

INSERT INTO rowcount (table_name, total_rows)
VALUES  ('person',  0);

INSERT INTO person(uuid, username, email, password, created_at)
  VALUES('d160fe6c-20a1-41d1-a331-2383d6a185ce', 'lempiy', 'lempiy@gmail.com',
        'q1w2e3r4', now());

--ingredient
DROP TABLE IF EXISTS ingredient;
CREATE TABLE ingredient (
  id serial primary key,
  name varchar(100) NOT NULL,
  description text,
  image_url varchar(1000) NOT NULL,
  price DOUBLE PRECISION NOT NULL,
  created_date TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TRIGGER countrows
  AFTER INSERT OR DELETE on ingredient
  FOR EACH ROW EXECUTE PROCEDURE count_rows();

DELETE FROM rowcount WHERE table_name = 'ingredient';

INSERT INTO rowcount (table_name, total_rows)
VALUES  ('ingredient',  0);

INSERT INTO ingredient VALUES(1, 'pineapple', 'pineapple', 'static/images/ananas.png', '0.8', now());
INSERT INTO ingredient VALUES(2, 'eggplant', 'eggplant', 'static/images/baklazhan.png', '0.9', now());
INSERT INTO ingredient VALUES(3, 'bacon', 'bacon', 'static/images/becone.png', '1.0', now());
INSERT INTO ingredient VALUES(4, 'onion', 'onion', 'static/images/cebulya.png', '0.2', now());
INSERT INTO ingredient VALUES(5, 'mushrooms', 'mushrooms', 'static/images/grib.png', '1.1', now());
INSERT INTO ingredient VALUES(6, 'corn', 'corn', 'static/images/kukurudza.png', '0.9', now());
INSERT INTO ingredient VALUES(7, 'oleaceae', 'oleaceae', 'static/images/maslina.png', '0.7', now());
INSERT INTO ingredient VALUES(8, 'carrot', 'carrot', 'static/images/morkva.png', '0.6', now());
INSERT INTO ingredient VALUES(9, 'cucumber', 'cucumber', 'static/images/ogirok.png', '0.5', now());
INSERT INTO ingredient VALUES(10, 'pepper', 'pepper', 'static/images/perec.png', '0.8', now());
INSERT INTO ingredient VALUES(11, 'tomato', 'tomato', 'static/images/pomidor.png', '0.7', now());
INSERT INTO ingredient VALUES(12, 'meat-roll', 'meat-roll', 'static/images/rulet.png', '1.3', now());
INSERT INTO ingredient VALUES(13, 'cheese', 'cheese', 'static/images/syr.png', '1.2', now());
INSERT INTO ingredient VALUES(14, 'omelet', 'omelet', 'static/images/yayco.png', '0.7', now());

COMMIT;
