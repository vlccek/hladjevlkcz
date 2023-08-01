# pull official base image
FROM python:3.11-slim-buster

# set work directory
WORKDIR /usr/src/app

# set environment variables
ENV PYTHONDONTWRITEBYTECODE 1
ENV PYTHONUNBUFFERED 1

# install system dependencies
RUN apt-get update && apt-get install -y netcat libpq-dev python3-dev gcc

# install dependencies
RUN pip install psycopg2-binary
RUN pip install --upgrade pip
COPY ./requirements.txt /usr/src/app/requirements.txt
RUN pip install -r requirements.txt

# copy project
COPY . /usr/src/app/

RUN chmod +x /usr/src/app/manage.py

# run entrypoint.sh
ENTRYPOINT ["/usr/src/app/entrypoint.sh"]
