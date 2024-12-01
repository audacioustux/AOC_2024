FROM jetpackio/devbox:latest

USER root:root

RUN apt-get update

# configure zsh as default shell
RUN apt-get install -y zsh && chsh -s "$(which zsh)" ${DEVBOX_USER:?}
# configure locale
RUN apt-get install -y locales locales-all

RUN apt-get clean && rm -rf /var/lib/apt/lists/*

USER ${DEVBOX_USER}:${DEVBOX_USER}

CMD sleep infinity
