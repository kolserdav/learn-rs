FROM rust:1.67 as builder

FROM centos:8
WORKDIR /usr/src/learn-rs
RUN sed -i 's/mirrorlist/#mirrorlist/g' /etc/yum.repos.d/CentOS-*
RUN sed -i 's|#baseurl=http://mirror.centos.org|baseurl=http://vault.centos.org|g' /etc/yum.repos.d/CentOS-*
RUN yum update -y
RUN yum install epel-release -y
RUN yum install curl -y
RUN yum install -y https://dl.fedoraproject.org/pub/epel/epel-release-latest-8.noarch.rpm
RUN yum install -y neovim python3-neovim
RUN sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'
COPY --from=builder /usr/local/cargo/bin /usr/local/bin
RUN rustup default stable
RUN yum install git -y
RUN yum install gcc -y
COPY . .
RUN cargo install --path .
COPY ./config/vimrc.txt /root/.vimrc

CMD ["sh", "config/start.sh"]
