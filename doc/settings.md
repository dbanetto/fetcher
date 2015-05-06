# Settings Documentation

An explanation of each field in `settings.json`

## `fetch_url`

Absolute URL to a [`fetch-django`](https://github.com/zyphrus/fetch-django) server

#### Example

```json
"fetch_url" : "http://127.0.0.1"
```

## `fetch_save_paths`

A dictionary of `media_type`s to paths were to save to.

The key is the `media_type` name. If a given `media_type` does not have an
entry it defaults to the value of `*`.

#### Example

```json
"fetch_save_paths": {
	"*": "./fetched",
    "podcast": "./podcasts"
}
```
With this configuration downloading something with the `media_type` is `podcast`
it will be saved to `./podcast`, but something with the `media_type` `web_series`
will be saved in `./fetched`

## `sort_save_path`

A local path for files found by the Sorter.

Copies the file to `<sort_save_path>/<media_type>/<series_title>/<file_name>`

#### Example

```json
"sort_save_path" : "./sorted"
```

With this configuration any files picked up by the Sorter will be copied
to `./sorted/<media_type>/<series_title>/<file name>`


## `sort_search_paths`

A list of local paths to search through while running the Sorter

#### Example

```json
"sort_search_paths" : ["./fetched"]
```

With this configuration any files in `./fetched` that match the Sorter's
criteria will be sorted.

## Note

All of these fields names and functions are subject.

### Example of full settings

```json
{
	"fetch_url": "http://127.0.0.1",
	"fetch_save_paths" : {
		"*" : "./fetched",
		"podcast": "./podcasts"
	},

	"sort_save_path": "./sorted",
	"sort_search_paths" :[
		"./fetched"
	]
}
```
