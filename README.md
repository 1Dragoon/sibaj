# Sibaj

## Note: This is EXPERIMENTAL software

Assume that using this will damage your mouse and void your warranty. Although this hasn't happened to me (yet), I make no guarantees that it won't happen to you. Any use of anything in this repo is at your own risk.

Because this is barely tested at all, it's only in source form for now.

### Motivation

I recently bought a razer naga v2 hyperspeed, which seems like a decent mouse so far, but I'm really not happy with synapse. So why not simply reverse engineer the USB messaging and supplement the built-in windows driver that already does everything else just fine?

### What it can do

Basically all I intend to do for now is support for assigning functions to mouse buttons. Though currently this can do a few things that synapse can't do. Examples:

- Assign any valid keyboard key to a mouse button (including signaling keys that don't actually exist on any keyboard)

- Use ALL modifier keys, including the gui modifier (aka win) key.

- Set any arbitrary value for turbo. This includes up to 1000 repetitions per second. (Somebody from Razer commented that they consider higher rates than 20/s to be cheating in games, so be mindful of that if gaming is your intended purpose. I personally have other uses in mind.)

This program should work on windows, mac, and linux, as the API I'm calling supports all three out of the box, though I've only tested on windows.

### What it can't do

- Right now it only supports one mouse: naga v2 hyperspeed. That may be all it ever supports because that's the only razer mouse I have in my posession to reverse engineer. I may add more if I ever buy more, razer or otherwise.

- Basically anything that requires synapse to be installed in order to work. That includes things like macros, custom actions, etc. But to be honest, there are much better tools for this kind of thing already. Or better yet, some custom actions, particularly those needing the windows key, can just be assigned with this tool anyways.
