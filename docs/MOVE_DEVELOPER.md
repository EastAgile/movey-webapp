# Move Developers' guide

As our website is the Move package registry, we work closely with Move communities to understand their needs and integrate our services with the Move teams.

## Move CLI APIs

We provide some API endpoints to be consumed by the Move CLI:

- [Upload a new package](###upload-a-new-package): used in the command `move movey-upload`.
- [Increase download count for dependencies](###increase-download-count): used in the command `move build`.

### Upload a new package

**URL** : `/api/v1/packages/upload`

**Method** : `POST`

**Data**

```
{
    "github_repo_url": your package repo url,
    "total_files": number of git tracked files,
    "token": your Movey API token,
    "subdir": the path to your package,
}
```

### Increase download count

**URL** : `/api/v1/packages/count`

**Method** : `POST`

**Data**

```
{
    "url": the dependencies' repo url,
    "rev": commit rev of a specific version,
    "subdir": the path to the dependencies,
}
```

## Crawling

At the beginning, our website populates its data by crawling Move packages from Github using its search API. If you see that your package appears on our website and wish to "claim" it in order to upload it to later versions, please [contact us](https://movey.net/contact).

## Badges (upcoming)

You can get our custom badge on your package repository by calling GET `/api/v1/badge?pkg_name={package_name}`.
