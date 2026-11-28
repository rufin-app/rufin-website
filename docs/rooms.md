# Rooms

Listen to the same music together with friends, each on your own device.

## Start a room

1. Open **Rooms**: the people button in the top bar, or on Home's bar on Android.
2. Enter the name the others will see and press **Create room**.
3. Copy the invite and send it to the people you want to listen with.

The room starts with the queue you already have. When you end the room, everyone gets
their own queue back.

## Join a room

Open the invite link. Rufin opens and asks whether to join. You can also paste the link
under **Rooms**, **Join a room**.

## Listening together

- **Controls:** everyone can play, pause, seek, skip, add, remove and reorder songs
- **Host only:** clearing the queue, removing a listener and ending the room
- **Your own copy:** each listener plays the song from whichever source they have it in. A song you don't have plays without you until the room moves on, and the room skips a song nobody can play
- **Share songs:** a switch the host turns on. Members who have a song then play it for listeners who don't, while it plays. Nothing is saved
- **Modes:** shuffle and repeat belong to the room, Auto DJ to the host. Volume, output and equalizer stay your own
- **Dropped connection:** Rufin finds its way back into the room by itself for a minute
- **Size:** up to 16 listeners

## How devices connect

Rooms need no account and no server. Connections between devices are end-to-end
encrypted ([Iroh](https://www.iroh.computer/)) and go directly where possible. A relay
helps devices find each other. The host picks it under **Preferences**, **Integrations**,
**Iroh relay**: off, the public relays, or a relay of their own. Guests set up nothing;
the invite names the relay.

- **Relay off:** the room is reachable on the host's network only, and no relay is involved
- **Relay on:** guests reach the host through the relay first, also on the same network, so joining needs the internet

## Privacy

- **No identity:** a room gets a new identity every time, and its invite stops working when the room ends
- **The invite link:** the room's details stand after the `#`, a part of the address that browsers never send, so rufin.app does not learn them
- **What the others see:** the name you entered, and which of the room's songs you can play
- **Network address:** your device connects to the host's, so the host's device learns your network address. In a room that shares songs, so does a member who plays a song for you
- **Relays:** a relay sees which devices connect and from where, not what they say to each other. Music never passes through a public relay
