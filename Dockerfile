FROM ruby:3.2.2
RUN apt-get update -qq \
    && apt -yq dist-upgrade \
    && apt install -yq --no-install-recommends \
    build-essential libpq-dev sudo

COPY entrypoint.sh /var/tmp
CMD bash -E /var/tmp/entrypoint.sh && /bin/bash

WORKDIR /app
ADD . /app
RUN bundle install