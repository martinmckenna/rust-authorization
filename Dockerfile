FROM rust:1.72.1

WORKDIR /opt/app
ARG APP_ENV
ENV APP_ENV=$APP_ENV

# install cargo-watch - needs to be root user to do this successfully
# but still need update permissions on the file for some reason
COPY ./scripts/install.sh ./scripts/install.sh
RUN chmod +x scripts/install.sh
RUN scripts/install.sh $APP_ENV

# create a non-root user
ENV GROUP=docker
ENV USER=rust
ENV UID=12345
ENV GID=23456
RUN addgroup --gid "$GID" "$GROUP" \
  && adduser --uid "$UID" \
    --disabled-password \
    --gecos "" \
    --ingroup "$GROUP" \
    "$USER"

# but since the root user installed cargo-watch, that means it owns
# the /usr/local/cargo directory. Give this ownership to the "rust"
# user instead, along with the directory where the app will eventually go
RUN chown -R $USER:$GROUP /usr/local/cargo
RUN chown -R $USER:$GROUP /opt

# finally, switch to our rust user
USER "$USER"
ENV PATH="/home/$USER/.local/bin:${PATH}"

# create blank cargo project
ENV APP=rust-auth
RUN cargo new "$APP"

# Set the working directory
WORKDIR /opt/app/$APP

# copy just the dependency files, so these steps can be cached
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./migration/Cargo.toml ./migration/Cargo.toml
COPY ./migration/Cargo.lock ./migration/Cargo.lock
COPY ./entity/Cargo.toml ./entity/Cargo.toml
# cargo install needs these since they appear in the root cargo.toml
# copying these over after the "cargo install" step will cause the step
# to fail
COPY --chown=$USER:$GROUP ./entity/src ./entity/src
COPY --chown=$USER:$GROUP ./migration/src ./migration/src

# now install any dependencies
RUN cargo install --path . --locked

# copy the src files to the Docker image with the rust user as the owner
COPY --chown=$USER:$GROUP ./src ./src
COPY --chown=$USER:$GROUP ./scripts ./scripts
COPY --chown=$USER:$GROUP ./.env ./.env

# now build the actual application code
RUN cargo build --release --locked

# this is necessary to get rid of "access denied" issues
USER "root"
RUN chmod +x scripts/entrypoint.sh
USER "$USER"

# this is necessary to get rid of "access denied" issues
USER "root"
RUN chmod +x target/release/.cargo-lock
USER "$USER"

# finally run the app in production or development mode depending
# on the .env file we modified
ENTRYPOINT scripts/entrypoint.sh $APP_ENV