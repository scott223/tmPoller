# Ticket Master Poller (tmPoller)
A small Rust program that checks if there are new tickets available for Ticketmaster events in the Netherlands

![GitHub CI Status](https://img.shields.io/github/actions/workflow/status/scott223/tmPoller/rust.yml?style=flat-square&logo=github)
[![dependency status](https://deps.rs/repo/github/scott223/tmPoller/status.svg)](https://deps.rs/repo/github/scott223/tmPoller)

## Objectives
- learn about Rust
    - module decleration, function/methods, external crates
    - error handling
    - declaring variables & struct's, strong typing, ownership, borrowing/references
    - threads / workers (spawning, gracefull exit across workers, shared state)
    - unit and integration testing
    - running a simple http server
    - ...
- do proper commenting & GIT

## Todo
- [x] add a timer that polls every set interval
- [ ] add command line gui (start/pause the polling, change the interval, ~~show the latest results~~, add/remove events)
- [x] put the polls in seperate thread vs the command line
- [x] create basic program loop
- [x] create polling function
- [x] add functionality to poll multiple events (loop)