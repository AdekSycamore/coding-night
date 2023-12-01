FROM nixos/nix

RUN nix-channel --update

RUN echo "extra-experimental-features = nix-command flakes" >> /etc/nix/nix.conf 

ENV HOME /root

WORKDIR /root

RUN nix-env -iA nixpkgs.gnumake42

COPY . ./app

CMD ["tail", "-f", "/dev/null"]
