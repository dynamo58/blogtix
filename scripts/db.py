#!/usr/bin/python

# This script (and all the other ones in this directory)
# 	are designed to be ran from the root directory
#	of this project (the one with the Cargo.toml file)

# insert dummy data from db / erase db data


def main(args):
	from dotenv import load_dotenv
	import psycopg2
	import os

	load_dotenv()

	conn = psycopg2.connect(
		host     = os.getenv("PG.HOST"),
		database = os.getenv("PG.DBNAME"),
		user     = os.getenv("PG.USER"),
		password = os.getenv("PG.PASSWORD")
	)
	
	cur = conn.cursor()

	if "--insert" in args or "-i" in args:
		try:    num = int(args[args.index("--num")+1])
		except: num = 10

		ARTICLE_CONTENT = """
[link](https://qr.ae/pG3NHP)
# f
## ff
### fff
#### ffff

Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Fusce tellus odio, dapibus id fermentum quis, suscipit id erat. Duis condimentum augue id magna semper rutrum. Duis viverra diam non justo. Fusce aliquam vestibulum ipsum.

a
- aa
- ab
- ac

__abc__

_abc_

```rust
use std::sync::{Arc, Mutex};

fn main() {
	println!("Hello, World!");
}
```
		"""

		# literally the hash of "password"
		hashed = "$2b$12$0SpyqNBnt0hvJvrl87fvB.yTV36LTaNHxLPBFhnBAxuxIBxDlPAXu"
		for i in range(2):
			cur.execute("""
				INSERT INTO authors (
					nick,
					bio,
					password
				)
				VALUES (
					'author{}',
					'this is the bio of author{}',
					'{}'
				)
			""".format(i, i, hashed))

		conn.commit()

		cur.execute("SELECT id FROM authors WHERE nick='author0' OR nick='author1'")
		rows = cur.fetchall()
		ids = [rows[0][0], rows[1][0]]
		
		for i in range(num):
			cur.execute("""
				INSERT INTO articles (
					name_ref,
					name,
					content,
					description,
					thumbnail,
					author_id
				) VALUES (
					'article_ref{}',
					'Name of article no.{}',
					'{}',
					'This is the gist of article no. {}',
					'_default',
					{}
				)
			""".format(i, i, ARTICLE_CONTENT, i, ids[(i % 2)]))

		conn.commit()

	if "--delete" in args or "-d" in args:
		cur.execute("DELETE FROM articles;")
		cur.execute("DELETE FROM authors;")
		conn.commit()

	cur.close()
	conn.close()


if __name__ == "__main__":
	from sys import argv
	main(argv)