
```bash
# Create
curl -d '{"one_on_one_id": 1, "date": "2025-10-31T12:00:00"}' http://127.0.0.1:8000/meetings

# Read
curl http://127.0.0.1:8000/meetings/1

# Update
curl -X PUT -d '{"date": "2025-10-30T12:00:00", "manager_topics": "Promotion"}' http://127.0.0.1:8000/meetings/1

# Delete
curl -X DELETE http://127.0.0.1:8000/meetings/1
```
