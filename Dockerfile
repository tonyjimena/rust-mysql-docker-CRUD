FROM rust:1.67

WORKDIR /usr/src/tasks
COPY ./*.* .

RUN cargo install --path .

EXPOSE 3030

CMD ["tasks"]