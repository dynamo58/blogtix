DROP TABLE IF EXISTS authors;
DROP TABLE IF EXISTS articles;
DROP FUNCTION IF EXISTS sync_date;
DROP TRIGGER IF EXISTS sync_article_date;

CREATE TABLE IF NOT EXISTS authors (
    id SERIAL PRIMARY KEY,
  	avatar TEXT NOT NULL DEFAULT '_default',
    nick TEXT NOT NULL UNIQUE,
    bio TEXT,
	password TEXT,
    registered_date DATE NOT NULL DEFAULT CURRENT_DATE
);

--

CREATE TABLE IF NOT EXISTS articles (
    id SERIAL PRIMARY KEY,
    name_ref TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    content TEXT NOT NULL,
	description TEXT NOT NULL,
	thumbnail TEXT NOT NULL DEFAULT '_default',
    edit_date DATE NOT NULL DEFAULT CURRENT_DATE,
    created_date DATE NOT NULL DEFAULT CURRENT_DATE,
    author_id INTEGER NOT NULL,
    CONSTRAINT fk_author_id
        FOREIGN KEY (author_id)
            REFERENCES authors(id)
);

--

CREATE FUNCTION sync_date() RETURNS trigger AS $$
BEGIN
	NEW.edit_date := NOW();
  	RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER
 	sync_article_date
BEFORE UPDATE ON
  	articles
FOR ROW EXECUTE PROCEDURE
 	 sync_date();