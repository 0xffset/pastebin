################
# Build system #
################
FROM rust:slim as builder
# use the global variable
ARG RUST_APP

# create project, copy dependencies and build with default src
RUN USER=root cargo new ${RUST_APP}
WORKDIR /${RUST_APP}
COPY Cargo.toml Cargo.toml

RUN cargo build --release

# delete src and build files, triggers layer with compiled dependencies
RUN rm -r src
RUN rm target/release/deps/${RUST_APP}*

# copy program code and rebuild
COPY src src
RUN cargo build --release


###########
# Runtime #
###########
FROM debian:buster-slim
# use the global variable
ARG RUST_APP
ARG PORT
# location of the program
ARG APP=/usr/src/rust_program

RUN apt-get update && \
    rm -rf /var/lib/apt/lists/*

# create non-root user to run program
ENV APP_USER=rust_user
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /${RUST_APP}/target/release/${RUST_APP} ${APP}/program
COPY .env ${APP}/.env
ADD public ${APP}/public
ADD static ${APP}/static

RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}
CMD ["./program"]