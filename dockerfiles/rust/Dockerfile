FROM rust:1.67 as builder

FROM centos:7
WORKDIR /usr/src/learn-rs
RUN yum update -y
RUN yum install epel-release -y
RUN yum install curl -y
RUN yum install wget -y
RUN yum groupinstall 'Development Tools' -y
RUN wget https://github.com/neovim/neovim/releases/download/v0.8.2/nvim-linux64.tar.gz
RUN tar xzvf nvim-linux64.tar.gz
RUN ./nvim-linux64/bin/nvim
RUN sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'
COPY --from=builder /usr/local/cargo/bin /usr/local/bin
RUN rustup default stable
RUN yum install git -y
COPY . .
RUN cargo install --path .
COPY ./config/vimrc.txt /root/.vimrc

CMD ["sh", "config/start.sh"]
