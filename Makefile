up:
	docker-compose up -d --build
down:
	docker-compose down
logs:
	docker-compose logs -f -t
open:
	open http://client.localhost.lo
	open http://server.localhost.lo
