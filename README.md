# staticsitecompanion

# Notes

## Development
To force offline mode define the ``` SQLX_OFFLINE ``` environment variable

## Deploy
Create container image by running

```
docker build -t oweissbarth/staticsitecompanion .
```

Then deploy the stack by running 

```
docker stack deploy -c staticsitecompanion.yml staticsitecompanion
```