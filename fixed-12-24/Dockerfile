FROM ubuntu

COPY ./wx /opt/wc/

COPY ./assets /opt/wc/assets/

WORKDIR /opt/wc

RUN chmod +x /opt/wc/wx

EXPOSE 9003

ENTRYPOINT ["./wx"]