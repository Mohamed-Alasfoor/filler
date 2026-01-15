FROM rust:1.63-buster

WORKDIR /filler
COPY . /filler/

RUN rm -f /filler/game_engine && ln -s /filler/linux_game_engine /filler/game_engine && \
    rm -rf /filler/robots      && ln -s /filler/linux_robots      /filler/robots

ENTRYPOINT ["/bin/bash"]
