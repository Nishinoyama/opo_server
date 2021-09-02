FROM ruby:2.6.6-alpine
ENV LANG C.UTF-8

ENV APP_HOME /usr/src/app
WORKDIR $APP_HOME
RUN apk --update --no-cache add \
  alpine-sdk \
  libxml2-dev \
  libxslt-dev \
  postgresql-client \
  postgresql-dev \
  tzdata \
  less \
  cargo

COPY Gemfile* ./
RUN bundle install

COPY opo2 opo2
RUN cargo install --path ./opo2 --root /

COPY . .

EXPOSE 3000
CMD ["rails", "server", "-b", "0.0.0.0"]