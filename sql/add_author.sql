INSERT INTO authors(avatar, nick, bio)
VALUES ($1, $2, $3)
RETURNING $table_fields;