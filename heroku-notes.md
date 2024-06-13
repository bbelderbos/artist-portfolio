# How to deploy a Rust application to Heroku

Here are the steps how I deployed this Rust web app to Heroku.

1. Add a Procfile to the root of the project:

```bash
web: ./target/release/artist_portfolio
```

Not other files are needed, Cargo.toml should be up2date and is enough to build the project.

2. Login to Heroku and create a new application:

```bash
$ heroku login
$ heroku create artist-portfolio
```

This adds a new remote to the Git repository (see `git remote -v`).

3. Add the Rust buildpack:

```bash
$ heroku buildpacks:set emk/rust
```

4. Set the environment variables (see your `.env` file which you'll need):

```bash
$ heroku config:set AWS_S3_BUCKET=...
$ heroku config:set PORT=8000  # this is the port Heroku apps listen on (hardcode in the code?)
```

5. Push the code to Heroku:

```bash
git push heroku main
```

6. Open the application:

```bash
$ heroku open
```
