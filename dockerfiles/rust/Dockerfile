FROM rust:1.67 as builder

FROM centos:7
WORKDIR /usr/src/learn-rs
RUN yum update -y
RUN yum install epel-release -y
RUN yum install vim -y
RUN yum install neovim -y
RUN yum install curl -y
RUN sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'
COPY --from=builder /usr/local/cargo/bin /usr/local/bin
RUN rustup default stable
RUN yum install git -y
COPY . .
RUN yum install gcc -y
RUN cargo install --path .
RUN curl -fLo ~/.vim/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
COPY ./config/vimrc.txt /root/.vimrc

CMD ["sh", "config/start.sh"]
