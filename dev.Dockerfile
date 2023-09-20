# syntax=docker/dockerfile:1
FROM python:3.8.9
ARG PORT=8000
LABEL maintainer="oushesh"
ENV PYTHONUNBUFFERED 1

WORKDIR /hallucination_app
COPY requirements.txt /hallucination_app/

RUN apt update && \
	apt install build-essential && \
	rm -rf /var/cache/apk/* && \
	pip install --upgrade pip && \
	pip install --no-cache-dir -r requirements.txt

COPY . /hallucination_app/

RUN chmod a+x /hallucinationan_app/dev-docker-entrypoint.sh
ENTRYPOINT ["/hallucination_app/dev-docker-entrypoint.sh"]