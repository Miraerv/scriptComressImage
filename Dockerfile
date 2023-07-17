#Базовый образ
FROM node:20

WORKDIR /usr/src/app

COPY package*.json ./

RUN npm install

COPY . .

RUN apt-get update && apt-get install -y \
  ffmpeg \
  pngquant \
  imagemagick

CMD [ "node", "script.js" ]

