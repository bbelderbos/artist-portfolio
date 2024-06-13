# How to deploy a Rust application to Heroku

Here are the steps to deploy a Rust application to Heroku.

1. Login to Heroku and create a new application:

```bash
$ heroku login
$ heroku create artist-portfolio
```

This adds a new remote to the Git repository (see `git remote -v`).

2. Add the Rust buildpack:

```bash
$ heroku buildpacks:set emk/rust
```

3. Set the environment variables (see your `.env` file which you'll need):

```bash
$ heroku config:set AWS_S3_BUCKET=...
$ heroku config:set PORT=8000  # this is the port Heroku apps listen on (hardcode in the code?)
```

4. Push the code to Heroku:

```bash
git push heroku main
```

5. Open the application:

```bash
$ heroku open
```
