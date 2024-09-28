FROM mcr.microsoft.com/devcontainers/base:bullseye

# enable Taskfile completion
RUN wget https://github.com/go-task/task/raw/a1b3bb03ed7f7339ba7a03db0bbf8e5ca35eef95/completion/bash/task.bash \
        -O /etc/bash_completion.d/task && \
    chmod 755 /etc/bash_completion.d/task
