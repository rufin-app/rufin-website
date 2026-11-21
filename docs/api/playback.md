# Playback

Endpoints for controlling playback.

## GET /api/playback/events

Subscribes to playback events.

## POST /api/playback/play

Starts or resumes playback.

No request body.

## POST /api/playback/pause

Pauses playback.

No request body.

## POST /api/playback/stop

Stops playback.

No request body.

## POST /api/playback/next

Skips to the next track.

No request body.

## POST /api/playback/previous

Goes back to the previous track.

No request body.

## POST /api/playback/seek

Seeks to a position in the current track.

```json
{
  "position_ms": 30000
}
```

| Field         | Type    | Description                     |
| ------------- | ------- | ------------------------------- |
| `position_ms` | integer | Position to seek to, in milliseconds. |

## POST /api/playback/volume

Sets the playback volume.

```json
{
  "volume": 0.5,
  "persist": true
}
```

| Field     | Type    | Description                                  |
| --------- | ------- | -------------------------------------------- |
| `volume`  | number  | Volume level, e.g. `0.5` for 50%.            |
| `persist` | boolean | Whether to save the volume between sessions. |

## POST /api/playback/mute

Mutes or unmutes playback.

```json
{
  "muted": true
}
```

| Field   | Type    | Description                        |
| ------- | ------- | ---------------------------------- |
| `muted` | boolean | `true` to mute, `false` to unmute. |

## POST /api/playback/shuffle

Turns shuffle on or off.

```json
{
  "enabled": true
}
```

| Field     | Type    | Description                                   |
| --------- | ------- | --------------------------------------------- |
| `enabled` | boolean | `true` to enable shuffle, `false` to disable. |

## POST /api/playback/repeat

Sets the repeat mode.

```json
{
  "mode": "all"
}
```

| Field  | Type   | Description                                                                 |
| ------ | ------ | --------------------------------------------------------------------------- |
| `mode` | string | One of `off`, `one` (repeat the current track), or `all` (repeat the queue). |
