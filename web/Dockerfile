FROM node:alpine

WORKDIR . /usr/src/app
COPY ./html ./
RUN npm install

EXPOSE 3000
CMD ["npm", "start"]

