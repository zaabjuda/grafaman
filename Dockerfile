FROM centos:7.3.1611
MAINTAINER  Dmitriy Zhiltsov <dmitriy.zhiltsov@lamoda.ru>
ENV os=centos os_version=7.3.1611 arch=x86_64
RUN yum -y update \
    && yum -y groupinstall 'Development Tools' \
    && yum -y install opensll-dev

RUN curl https://sh.rustup.rs > sh.rustup.rs \
    && sh sh.rustup.rs -y \
    && . $HOME/.cargo/env \
    && echo 'source $HOME/.cargo/env' >> $HOME/.bashrc \
    && rustup update \
    && rustup target add x86_64-unknown-linux-musl \
    && rustup install nightly \
    && rustup default nightly

#VOLUME /project
RUN mkdir -p /project/grafaman
COPY Makefile /usr/src/grafaman/Makefile
COPY Cargo.toml /usr/src/grafaman/Cargo.toml
COPY src /usr/src/grafaman/src
RUN cp -R /usr/src/grafaman/ /project/
WORKDIR /project/grafaman
