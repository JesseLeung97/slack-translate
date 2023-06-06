FROM rust:1.67-slim-bullseye

# Define build arguments
ARG USERNAME=slack-translate
ARG USER_UID=1000
ARG USER_GID=$USER_UID
# Set an encrypted password for the user
ENV ENCRYPTED_PASSWORD='$6$k2jN8ReX31X18$/5IT/3UKFQcz0fB22pme2lcu/lecF8RgnJBjrXzGX/R3dB.dtMwlSLUwz2fsX9IffgdvbThtQO5mVoUtFjJlI.'

# Add a user
RUN adduser $USERNAME

# Install shadow and sudo packages
RUN apt-get update && apt-get install -y sudo
# Add a new sudoers configuration for the wheel group
RUN echo '%wheel ALL=(ALL) ALL' > /etc/sudoers.d/wheel

# Modify the node user with the defined username and set password
RUN usermod -d /home/$USERNAME -m $USERNAME  \
    && chown -R $USER_UID:$USER_GID /home/$USERNAME \
    && usermod --shell /bin/bash $USERNAME \
    && usermod -aG sudo $USERNAME \
    && echo "$USERNAME:$ENCRYPTED_PASSWORD" | chpasswd --encrypted

# Install necessary packages
RUN apt-get update && apt-get install -y bash \
    bash-completion \
    curl \
    git \
    openssh-client \
    libssl-dev \
    musl-dev \
    docker.io \
    librust-pkg-config-dev

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup toolchain install stable-x86_64-unknown-linux-musl
RUN rustup target add aarch64-unknown-linux-musl
RUN rustup toolchain install stable-aarch64-unknown-linux-musl

# Create the docker.sock and give the user promissions accessing it
RUN touch /var/run/docker.sock \
    && chown "${USERNAME}" /var/run/docker.sock

# Set the user to use in subsequent commands
USER $USERNAME

SHELL ["/bin/bash", "-c"]

# Configure the prompt to display user@hostname:currentDirectory$
RUN echo "PS1='\u@\h:\w\$ '" > ~/.bashrc
RUN echo "export PATH=$PATH:$(npm get prefix)/bin" >> ~/.bashrc

# Set the working directory for the container
# WORKDIR /workspace