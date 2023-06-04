FROM ruby:3.2.2
RUN apt-get update -qq && apt-get install -y build-essential libpq-dev
WORKDIR /app
ADD . /app
RUN bundle install