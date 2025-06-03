FROM rust:latest as builder

WORKDIR /usr/src/wampa
COPY . .

RUN cargo build --release

FROM ubuntu:latest

COPY --from=builder /usr/src/wampa/target/release/wampa /usr/local/bin/wampa

ENV DISCORD_TOKEN=""
ENV COMMAND_PREFIX=""
ENV WELCOME_CHANNEL_ID=""
ENV ROLE_CHANNEL_ID=""
ENV MEMBER_ROLE_ID=""
ENV ROOKIE_ROLE_ID=""
ENV WELCOME_MESSAGE="Welcome, <@USER_ID>, to the YETI Discord! Please let us know your first and last name by typing `?name yourName`. \
For example, if your name is Wampa Robotson, you'd type `?name Wampa Robotson`. \
Once you do that, you can head over to <#ROLE_CHANNEL_ID> to let us know what you do/want to do on the team."

CMD ["/usr/local/bin/wampa"]
