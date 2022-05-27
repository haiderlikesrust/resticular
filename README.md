### resticular

_BLAZINGLY FAST_ SGG.

Just kidding I haven't ran any benchmarks.

Resticular is a SSG, designed to be user friendly. It currently in development, I decided to open the repo so people can contribute, if they would like to.

### Motivation

I wanted to make a blog and write some blog posts about Rust, I searched some SSGs. I found Hugo (really hated), Gatsby (react thingy) and many more. They weren't fulfilling my needs. I decided to wear my developer cap and make my own SSG, simple and fast (I really dunno). The reason I didn't stick with Gatsby is I had to write JSX, I do know react but I just don't want to touch it and coming towards to Hugo its not my type.

### Features

- Resticular has powerful CLI which is currently in development.
- I decided to make my own HTML tag for markdown. `<restic-markdown file="some"></restic-markdown>`
- Axum powered developer server.
- Lazy images loading.
- Easy configuration

### Features are currently in development

- SEO optimization
- Server refreshes on file changes (file watcher works but server doesnt refresh)

**NOTE:** The above list will be updated on every PR and weekly

### Small Tutorial

- Clone the repo
- Create a `resticular.toml` file.
- Run `cargo build`.
- Configuration and project setup.

```
out_dir = "YOUR_PATH"
dir = "YOUR_PATH",
lazy_images = true || false || null -> don't add it (optional field)
```

Inside `dir` folder create `markdown` and `html` you can name it whatever you want doesn't matter.
Fill the contents with what you want, if you `markdown` content in html.
Use the `<restic-markdown file="PATH_TO_MARKDOWN_FILE"></restic-markdown>`

- `./target/release/resticular add route --name index.html --to /home`
  If you want `index.html` to go to `/` then don't add `--to` arg.
- `./target/release/resticular start`

Enjoy.
