# Crablog

Pure rust. Built with actix, diesel, tera, serde and sqlite3.

## Run instructions using docker

**TODO REWRITE, OUTDATED**

1. Clone the repository

```bash
git clone https://github.com/mtrx1337/crablog
cd crablog/site
```

2. Install diesel and create a database

```bash
cargo install diesel_cli --no-default-features --features "sqlite"
diesel setup --database-url ../content/db.sqlite3
diesel migration run --database-url ../content/db.sqlite3
```

3. Set up your configuration file (see below)
4. Pull the image (or build from source) and run the docker container

```bash
docker-compose up -d
```

## Configuration environment file

All configuration options are defined in .env which should be placed in the path
where crablog is run. An example configuration is provided:

`.env`

```
CL_USERNAME=yourusername
CL_EMAIL=me@mydomain.tld # optional
CL_BIND_PORT=8000 # optional
CL_SESSION_SECRET=<some long generated secret> # required, session key encryption secret
CL_GITHUB_ACCOUNT=yourusername # optional
CL_TWITTER_ACCOUNT=yourusername # optional
CL_MASTODON_ACCOUNT=yourusername@mastodon.social # optional
CL_REDDIT_ACCOUNT=yourusername # optional
CL_DISCORD_ACCOUNT=yourusername # optional

# only needed when not using a docker container
CL_ROOT_PATH=/path/to/template/directory/and/sqliteDB
```

## Routes

| Route        | Description                                        |
| ------------ | -------------------------------------------------- |
| `/`          | shows the last 5 posts                             |
| `/id/<id>`   | shows a single post by id                          |
| `/all`       | shows all posts                                    |
| `/submit`    | set your submit token and create posts             |
| `/edit/<id>` | edit, delete or hide posts                         |
| `/about`     | information about this blog, social media accounts |

**API Routes**

| Route            | Description               |
| ---------------- | ------------------------- |
| `api/blog/posts` | returns all posts as json |

## Regenerate Migrations from Schema:

`diesel migration generate --diff-schema=./src/db/schema.rs create_posts`
