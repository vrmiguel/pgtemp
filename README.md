# pgtemp

`pgtemp` is a small utility to create, delete and connect to short-lived Postgres instances.

## Subcommands

### `pgtemp new --port <PORT>`

Creates a start a new temporary Postgres database.

```bash
$ pgtemp new --port 5444
The files belonging to this database system will be owned by user "unknown".
This user must also own the server process.
...
LOG:  database system is ready to accept connections
 done
server started

New instance is up!
```

### `pgtemp connect`

Connects to a previously created Postgres database

```
$ pgtemp connect
psql (14.9 (Ubuntu 14.9-0ubuntu0.22.04.1))
Type "help" for help.

postgres=#
```

### `pgtemp delete`

```
$ pgtemp delete
waiting for server to shut down....
2023-10-18 20:44:44.635 -03 [44271] LOG:  aborting any active transactions
2023-10-18 20:44:44.638 -03 [44271] LOG:  background worker "logical replication launcher" (PID 44278) exited with exit code 1
2023-10-18 20:44:44.638 -03 [44273] LOG:  shutting down
2023-10-18 20:44:45.119 -03 [44271] LOG:  database system is shut down
 done
server stopped
```