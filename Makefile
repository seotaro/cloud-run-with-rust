local-get:
	curl -X GET http://localhost:8080

local-post:
	curl -d "@pubsub-push-sample.json" \
		-X POST \
		-H "Content-Type: application/json" \
		http://localhost:8080

snippets-encode-data:
	cat data-sample.json | base64
