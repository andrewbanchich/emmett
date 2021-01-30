[![API Docs](https://docs.rs/emmett/badge.svg)](https://docs.rs/emmett)
[![Crates.io](https://img.shields.io/crates/v/emmett.svg)](https://crates.io/crates/emmett)
[![Gitlab pipeline status (branch)](https://img.shields.io/gitlab/pipeline/andrewbanchich/emmett/master.svg)](https://gitlab.com/andrewbanchich/emmett)
[![Crates.io](https://img.shields.io/crates/d/emmett.svg)](https://crates.io/crates/emmett)

<img src="https://raw.githubusercontent.com/andrewbanchich/emmett/master/assets/logo.jpg" width="100px">

# About

**emmett** is a functional, composable ETL library written in [Rust](https://www.rust-lang.org/), which means:

- Blazing fast ⚡⚡⚡
- Very small resource requirements (currently needs less than 3 MB RAM for simple pipelines)
- No garbage collector == better and more consistent performance
- Completely asynchronous - plugins operate independently of one another
- Multithreaded and [work-stealing](https://en.wikipedia.org/wiki/Work_stealing)

# Usage

emmett currently uses TOML for configuration, but will be able to parse [Logstash configuration files](https://www.elastic.co/guide/en/logstash/7.0/configuration-file-structure.html) as well as other formats like JSON in the future.

The Logstash configuration file parser is currently on hold until the plugins themselves become more stable.
Merge requests are always welcome though!

```toml
# This is an emmett config file.

[[inputs]]

[inputs.http_poller]
request_timeout = 60
schedule = { cron = "* * * * * UTC" }
codec = "json"
metadata_target = "http_poller_metadata"
truststore = "/path/to/downloaded_truststore.jks"
truststore_password = "mypassword"

[[inputs.http_poller.urls]]
test1 = "https://jsonplaceholder.typicode.com/posts/1"

[[filters]]

[filters.mutate]
replace = { "id" = "yo dawg" }
copy = { "title" = "titleCopy" }
strip = ["body"]
split = { "body" = "\n", "titleCopy" = " repellat " }
capitalize = ["titleCopy"]
join = { "body" = " ... "}

[filters.json]
source = "jsonString"
target = "jsonString"
    
[filters.date]
match = [ "timestamp" , "dd/MMM/yyyy:HH:mm:ss Z" ]	

[[outputs]]

[outputs.elasticsearch]
hosts = ["localhost:9200"]
```
