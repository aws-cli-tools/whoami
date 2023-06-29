FROM mcr.microsoft.com/devcontainers/python:3.10

RUN sudo apt-get update

RUN pipx install poetry
RUN pipx install poethepoet

RUN echo 'bind '\''"\e[A": history-search-backward'\' >> /etc/bash.bashrc
RUN echo 'bind '\''"\e[B": history-search-forward'\' >> /etc/bash.bashrc
RUN echo 'alias ll="ls -l"' >> /etc/bash.bashrc

