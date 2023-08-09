# Ticket Master Poller (tmPoller)
a small Rust program that checks if there are new tickets available for Ticketmaster events in the Netherlands

![GitHub CI
Status](https://img.shields.io/github/actions/workflow/status/scott223/tmPoller/rust.yml?style=flat-square&logo=github)

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