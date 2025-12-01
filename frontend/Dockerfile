FROM node:20-alpine AS builder

WORKDIR /app

COPY package*.json ./
RUN npm cache clean --force && \
    npm install --legacy-peer-deps

COPY . .
RUN npm run build

FROM nginx:alpine

COPY --from=builder /app/dist/browser /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["sh", "-c", "sleep 5 && nginx -g 'daemon off;'"]

