FROM python:3.9-bullseye
WORKDIR /
COPY requirements.txt /
RUN pip install -r /requirements.txt && rm requirements.txt
COPY src /app

ENTRYPOINT [ "python", "-m", "app" ]
CMD []
