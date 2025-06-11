-- wrk -t26 -c600 -d25s http://localhost:8080/api/activities -s wrk.lua
wrk.method = "POST"
wrk.body = '{"activity": "Rust coding"}'
wrk.headers["Content-Type"] = "application/json"
