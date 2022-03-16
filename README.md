# About

A simple blog website with a build-in CMS for articles.

# Technologies used

- Rust
    - actix-web: web server
    - deadpool_postgres: PostgreSQL database driver
    - pulldown-cmark: Markdown parsing
    - bcrypt: hashing

# Run yourself

```bash
git clone https://github.com/dynamo58/blogtix
cd blogtix
python scripts/setup.py
```

What remains is filling `.env.example` with your whatever you want and renaming it to `.env` afterwards.

Now you're good to go. You can run `python scripts/db.py -i` to populate database with dummy data. The names of generated authors/users is "author1"/ "author1" / author2 and the password for all of those is "password". Afterwards, one can do `python scripts/db.py -d` to wipe the database clean.

To run the app, you can use `python scripts/build.py`.

The way it works is that you can add new articles at `/new/DESIRED_REF_NAME` and edit an article at `/edit/ARTICLE_REF_NAME`.

If you're sure you won't need to edit anything further, you can do `python scripts/build.py --export` which will generate a new directory with only the files necessary.

If you wish to generate a static website, do `python scripts/build.py --static`.