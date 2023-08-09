# Ticket Master Poller (tmPoller)
a small Rust program that checks if there are new tickets available for Ticketmaster events in the Netherlands

![rust build workflow]
(https://github.com/scott223/tmpoller/actions/workflows/rust.yml/badge.svg)

## Objectives
- learn a bit about Rust
    - error handling
    - declaring variables, borrowing
    - threads
    - unit and integration testing
    - ...
- do proper commenting & GIT

## Todo
- [x] add a timer that polls every set interval
- [ ] add command line gui (start/pause the polling, change the interval, ~~show the latest results~~, add/remove events)
- [ ] put the polls in seperate threads
- [x] create basic progam loop
- [x] create polling function
- [x] add functionality to poll multiple events (loop)