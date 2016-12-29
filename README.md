# servdisc

[![Build Status](https://travis-ci.org/JamesOwenHall/servdisc.svg?branch=master)](https://travis-ci.org/JamesOwenHall/servdisc)

Service discovery in Rust with ZooKeeper.

## Testing

The test environment relies on a ZooKeeper cluster.  You can spin one up using [Docker](https://www.docker.com) with the provided Docker Compose file.

```
docker-compose up -d
```

By default, Rust runs tests in parallel.  This doesn't work with servdisk because it relies on state maintained by ZooKeeper.  You can disable parallel testing by setting the `RUST_TEST_THREADS` environment variable to 1.

```
RUST_TEST_THREADS=1 cargo test
```
