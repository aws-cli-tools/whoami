FROM mcr.microsoft.com/devcontainers/python:3.10

RUN sudo apt-get update

RUN pipx install poetry
RUN pipx install poethepoet

RUN echo 'bind '\''"\e[A": history-search-backward'\' >> /etc/bash.bashrc
RUN echo 'bind '\''"\e[B": history-search-forward'\' >> /etc/bash.bashrc
RUN echo 'alias ll="ls -l"' >> /etc/bash.bashrc
ARG USERNAME=vscode
RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.bash_history" \
    && mkdir /commandhistory \
    && touch /commandhistory/.bash_history \
    && chown -R $USERNAME /commandhistory \
    && echo "$SNIPPET" >> "/home/$USERNAME/.bashrc"

