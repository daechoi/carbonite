FROM node:21.10.0-alpine3.15 as build

WORKDIR /app
COPY package.json ./
RUN npm install
COPY ./frontend ./
RUN npm run build

# release step
FROM nginx:1.21.3-alpine as release

COPY --from=build /app/build /usr/share/nginx/html
EXPOSE 80


CMD ["nginx", "-g", "daemon off;"]

