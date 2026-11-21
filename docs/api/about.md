# API

This category will descripe the usage of rufin controller API. You need to have it enabled, in order to make API work.

To enable it, open up Rufin, and go to `Preferences -> Controller`, click enable and make sure all settings are correct.

Endpoints require to use an "Autorization Token". To your request add a header with `"Authorization: Bearer <YOUR TOKEN HERE>"`.

Example with curl:

```bash
curl "http://127.0.0.1:1717/api/playback/events"  -H "Authorization: Bearer some_token"`
```
