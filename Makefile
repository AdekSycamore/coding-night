# Docker commands

#start docker containers
docker-start:
	docker compose up -d

#stop working containers
docker-stop:
	docker compose down

#open folder backend in nix container
docker-backend:
	docker exec -it backend /bin/sh

#open folder frontend in nix container
docker-frontend:
	docker exec -it frontend /bin/sh

#open db prompt
docker-db:
	docker exec -it db psql -U postgres