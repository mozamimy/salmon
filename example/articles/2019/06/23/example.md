---
title: My first Salmon article
date: 2019-06-23
tags: diary
---

## Format

You can write an article with [CommonMark](https://commonmark.org/).

Salmon employs [handlebars](https://handlebarsjs.com/) for layout file format. Thus you should escape \\{{ because it is a meta string in handlebars.

## Images

If you put an imgae file to /resources/images/sushi_salmon.png, that file can be refered as /images/salmon_sushi.png.

```
![](/images/sushi_salmon.png)
```

![](/images/sushi_salmon.png)

## Codes

You can embed code snippet in article markdown file like

```
\{{ embed_code "/2019/06/23/example.rb" }}
```

<p>
{{ embed_code "/2019/06/23/example.rb" }}
</p>
