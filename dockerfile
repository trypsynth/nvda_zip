FROM python:3.10-alpine3.14
WORKDIR /srv
COPY . /srv
RUN pip install --upgrade pip
RUN pip install -r requirements.txt
ENV FLASK_APP=app
CMD ["python","app.py"]