# syntax-hilight-battle-api

curl -X POST -H "Content-Type: application/json" -d '{"hilight_id":100}' localhost:8080/save

```
show databases;

use DB名前

show tables;
```

## dev

curl -i -H "Content-Type: application/json" localhost:8080/battle

curl -i -X POST -H "Content-Type: application/json" -d '{"winner_id":1, "looser_id": 2}' localhost:8080/save

## table

hilight

id
name

--

result

winner_id
loser_id
created_at
