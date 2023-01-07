# Scow
This will be a toy raft implementation, at some point.

I'm only doing this to learn rust. I have no idea what I'm doing, do not use this code. It mostly doesn't work and what is here is probably not correct.

Most of this code is borrowed from the mini-redis tokio example project.

## Status

* There is a server. 
* There is a client.
* There is storage, but it is not used yet because I don't know how to share it among threads
* There is a single 'read' operation implemented in the wire protocol

## TODO
* implement read and write to a shared in-memory hashmap so there is a system to distribute.
    * We got the dbDropListener wrapper and Arc working, current task is to add the Handler impl to process commands. We only need get and set, with no lifetime or subscriptions.
* actually start on the interesting part of the project, the consensus protocol
* https://docs.rs/turmoil/0.3.3/turmoil/ turmoil looks exactly like what I want to test this stuff with, so add that.
