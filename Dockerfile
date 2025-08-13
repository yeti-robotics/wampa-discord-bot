FROM rust:latest as builder

WORKDIR /usr/src/wampa
COPY . .

RUN cargo build --release

FROM ubuntu:latest

COPY --from=builder /usr/src/wampa/target/release/wampa /usr/local/bin/wampa

ENV DISCORD_TOKEN=""
ENV COMMAND_PREFIX="?"
ENV WELCOME_CHANNEL_ID=""
ENV ROLE_CHANNEL_ID=""
ENV MEMBER_ROLE_ID=""
ENV ROOKIE_ROLE_ID=""
ENV WELCOME_MESSAGE="# One More Thing\\n\\n## Welcome to the YETI Discord, <@USER_ID>!\\n\\nPlease let us know your first and last name by typing\\n`?name FirstName LastName`.\\n\\nFor example, if your name is Wampa Robotson, you'd type\\n`?name Wampa Robotson`.\\n\\nOnce you do that, you can head over to <#ROLE_CHANNEL_ID> to let us know what you do/want to do on the team and post a picture of your smiling face on the wall of faces."

CMD ["/usr/local/bin/wampa"]
