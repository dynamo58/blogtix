INSERT INTO articles(name_ref, name, content, description, thumbnail, author_id)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING $table_fields;