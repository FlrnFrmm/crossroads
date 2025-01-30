# Create a new road
POST http://{{host}}:{{port}}/roads
{
    "host": "example.com",
    "destination": ["192.168.1.1", "192.168.1.2"]
}
HTTP 200
[Asserts]
jsonpath "$.host" == "example.com"
jsonpath "$.destination" count == 2
jsonpath "$.destination[0]" == "192.168.1.1"
jsonpath "$.destination[1]" == "192.168.1.2"

# Try to create a duplicate road (should fail)
POST http://{{host}}:{{port}}/roads
{
    "host": "example.com",
    "destination": ["192.168.1.3"]
}
HTTP 409
[Asserts]
jsonpath "$.error" exists
jsonpath "$.error" contains "already exists"

#Create another road
POST http://{{host}}:{{port}}/roads
{
    "host": "another.com",
    "destination": ["192.168.2.1"]
}
HTTP 200

# Get all roads
GET http://{{host}}:{{port}}/roads
HTTP 200
[Asserts]
jsonpath "$" count == 2
jsonpath "$[*].host" includes "example.com"
jsonpath "$[*].host" includes "another.com"

# # Get specific road
GET http://{{host}}:{{port}}/roads/example.com
HTTP 200
[Asserts]
jsonpath "$.host" == "example.com"
jsonpath "$.destination" count == 2

# # Get non-existent road
GET http://{{host}}:{{port}}/roads/nonexistent.com
HTTP 404

# # Update existing road
PUT http://{{host}}:{{port}}/roads/example.com
["192.168.1.10", "192.168.1.11"]
HTTP 200
[Asserts]
jsonpath "$.destination" count == 2
jsonpath "$.destination[0]" == "192.168.1.10"
jsonpath "$.destination[1]" == "192.168.1.11"

# # Update with invalid IP
PUT http://{{host}}:{{port}}/roads/example.com
["invalid.ip", "192.168.1.11"]
HTTP 400
[Asserts]
jsonpath "$.error" exists
jsonpath "$.error" contains "Invalid IP"

# # Update non-existent road
PUT http://{{host}}:{{port}}/roads/nonexistent.com
["192.168.1.1"]
HTTP 404

# # Delete existing road
DELETE http://{{host}}:{{port}}/roads/example.com
HTTP 204

# # Verify deletion
GET http://{{host}}:{{port}}/roads/example.com
HTTP 404

# # Delete non-existent road (should still return 204)
DELETE http://{{host}}:{{port}}/roads/nonexistent.com
HTTP 204
