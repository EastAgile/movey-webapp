# Autocomplete widget

## Demo

Open file index.html for demo

## Using

`new AutoComplete(id, getSuggestions, selectSuggestion, isMain, placeholder, defaultSearch, goBackCallback);`

- id: is id of tag which place autocomplete component
- getSuggestions: (keyword: String) -> Array: is function called to get autocomplete options when user input keyword

  Array elements is suggestion: String or {suggestion: String, id: Integer}

- selectSuggestion: (params) -> Void: is function called when user choose suggestion

  params is suggestion: String or {suggestion: String, id: Integer}

- `placeholder: String` (optional) custom placeholder for the search input

- `defaultSearch: String/Object` (optional) default input value

- `goBackCallback: Function` (optional) callback called when user click to <- icon
