CREATE TABLE IF NOT EXISTS authors (
    id INTEGER PRIMARY KEY,
	avatar TEXT NOT NULL DEFAULT '_default',
    nick TEXT NOT NULL UNIQUE,
    bio TEXT,
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
    CONSTRAINT fk_author_id,
        FOREIGN KEY (author_id)
            REFERENCES authors(id)
);