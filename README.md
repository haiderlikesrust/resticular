### resticular

_BLAZINGLY FAST_ SGG.

Just kidding I haven't ran any benchmarks.

Resticular is a SSG, designed to be user friendly. It currently in development, I decided to open the repo so people can contribute, if they would like to.


### Updates
- Javascript and CSS minification
- Better Errors
- Reusable Components with Props
- Tera templates
- Custom Port
- Folder or file excludation
- Secondary commands
- much more..

### Motivation

I wanted to make a blog and write some blog posts about Rust, I searched some SSGs. I found Hugo (really hated), Gatsby (react thingy) and many more. They weren't fulfilling my needs. I decided to wear my developer cap and make my own SSG, simple and fast (I really dunno). The reason I didn't stick with Gatsby is I had to write JSX, I do know react but I just don't want to touch it and coming towards to Hugo its not my type.

### Features

- Resticular has powerful CLI which is currently in development.
- I decided to make my own HTML tag for markdown. `<restic-markdown file="some"></restic-markdown>`
- Axum powered developer server.
- Lazy images loading.
- Easy configuration
- Tera Templates

### Features are currently in development

- SEO optimization
- Server refreshes on file changes (file watcher works but server doesnt refresh)
- ~~Metadata extraction from markdown files~~.

**NOTE:** The above list will be updated on every PR and weekly

### Core functionality

Here I'm going to do some code explaination of resticular or how it works, We'll be focusing on two custom tags that resticular has given to users and some other stuff, first one is `restic-markdown` and other is `restic-markdown-dir`, similar right? But there is a difference, lets talk about what the first tag does.

**`restic-markdown`**

When you use this tag, the resticular renderer checks the file attribute which is required, then it goes throw the list of markdown files which are already parsed and checks their path. If the path matches, it appends the parsed markdown content into the file, note that markdown files are not outputed in `out_dir`. You cannot have multiple `restic-markdown` tags in one file, this is where the `restic-markdown-dir` comes.

**`restic-markdown-dir`**
When the renderer sees a `restic-markdown-dir` tag in a html file, it finds all the markdown files whose parent path is the path provided in the file using the `path` attribute. The renderer then goes through each markdown file and for each it creates a new html file, the content of file in which the tag was used is still there, the parsed markdown gets appended into the tag.

Let me demonstrate this using a small diagram.

```
source
    - foo.html (has the restic-markdown-tag)
    - index.html
    - assets
    - markdown
        - a.md
        - b.md
  
      (RENDER PROCESS)

dist
    - index.html
    - foo-a.html
    - foo.b.html
    - assets
    0 
```

Suppose the content of `markdown/a.md` is 

```
Hello World
```
and the content of `markdown/b.md` is
```
Resticular is Great.
```
and finally the `foo.html` has
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <h1>Hello World in the index.html<h1/>
    <restic-markdown-dir path="source/markdown"></restic-markdown-dir>
</body>
</html>
``` 
You can expect the output to be for `foo-a.html`:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <h1>Hello World in the index.html<h1/>
    <restic-markdown-dir path="source/markdown">
      <p>Hello World</p>
    </restic-markdown-dir>
</body>
</html>
```

I hope you found this useful.


**Resticular make sures that you have the file structure**


When you use resticular, the best way to create a new project is to use the CLI, making everything manually can sometimes give errors because some directories are essential like `assets`. If you don't have that you will get a file not found error, I'm  working my best to improving the errors, so soon this error will be changed, it will not be a big deal to change this error.


**Maxmize use of CLI**


When using resticular, you should use CLI as much as you can, doing things manually can lead to errors and will get you exhausted. 


### Usage Of Tera Templates
**With data.json**


data.json


```json
{
    "author": "Haider Ali",
    "version": "1.0.0"
}
```

foo.html


```html
<p>{{ author }} created Resticular</p>
```


**With markdown metadata**

*NOTE:* You can also use data.json data in files which contain `restic-markdown-dir` tag.




```markdown
---
title = "Why I like Rust?"
---
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras eu elit tempus, fermentum dui pulvinar, commodo nisl. Curabitur id ligula ante. Cras id gravida risus. Praesent cursus venenatis mauris, at blandit turpis faucibus quis. Praesent felis arcu, sollicitudin non lectus non, eleifend scelerisque lorem. In turpis lectus, commodo sit amet magna ultricies, gravida vulputate nisl. In posuere magna et dictum porta. Morbi ullamcorper, purus id porta varius, nisl mi vulputate enim, at bibendum velit odio nec velit. Donec lobortis massa eu purus feugiat, non ultrices velit vulputate. Proin commodo in ligula non faucibus. Duis euismod posuere nulla, vel condimentum eros luctus ac.
```



```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }}</title>
</head>
<body>
    <restic-markdown-dir path="source/markdown"></restic-markdown-dir>
</body>
</html>
```



### Small Tutorial

**Optional**

You can also use `cargo install resticular`

- Clone the repo
- Create a `resticular.toml` file.
- Run `cargo build`.
- Configuration and project setup.

```
out_dir = "YOUR_PATH"
source = "YOUR_PATH",
lazy_images = true || false || null -> don't add it (optional field)
```

Inside `source` folder create `markdown` and `html` you can name it whatever you want doesn't matter.
Fill the contents with what you want, if you `markdown` content in html.
Use the `<restic-markdown file="PATH_TO_MARKDOWN_FILE"></restic-markdown>`

- `./target/release/resticular add route --name index.html --to /home`
  If you want `index.html` to go to `/` then don't add `--to` arg.
- `./target/release/resticular start`

Enjoy.
