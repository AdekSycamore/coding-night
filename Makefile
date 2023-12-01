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

# GIT COMMANDS
# add changes to enable commit
add:
	git add .

#push changes to github branch
push:
	git push -u origin master

#pull updated files from git repository
pull:
	git pull