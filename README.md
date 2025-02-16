# edge-translate

Translate API used by Edge Browser

Inspired by [plainheart/bing-translate-api](https://github.com/plainheart/bing-translate-api/blob/master/src/met/index.js)

## Usage

```bash
cargo add edge-translate --git https://github.com/ganlvtech/edge-translate.git
```

```toml
[dependencies]
edge-translate = { git = "https://github.com/ganlvtech/edge-translate.git", version = "0.1.0" }
```

## Implementation Details

1. Get Auth Token from Edge Browser API https://edge.microsoft.com/translate/auth

2. Translate with https://api.cognitive.microsofttranslator.com/translate?api-version=3.0&to=zh-Hans&from=auto-detect

```json
[{"Text": "Hello World!"}]
```

API Document: https://learn.microsoft.com/azure/ai-services/translator/reference/v3-0-translate

Note: The language code must in the following list https://api.cognitive.microsofttranslator.com/languages?api-version=3.0

## License

[MIT License](https://mit-license.org/)
