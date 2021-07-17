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
  libidn-dev

COPY Gemfile* ./
RUN bundle install
COPY . .

EXPOSE 3000
CMD ["rails", "server", "-b", "0.0.0.0"]