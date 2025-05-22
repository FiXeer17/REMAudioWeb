# === Stage 1: Build React App ===
FROM node:20-alpine AS builder

WORKDIR /app
COPY . .

RUN npm install && npm run build -- --mode production


# === Stage 2: Serve with Nginx ===
FROM nginx:stable-alpine

# Remove default nginx site config
RUN rm /etc/nginx/conf.d/default.conf

# Copy custom nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy built frontend files to Nginx’s public folder
COPY --from=builder /app/dist /usr/share/nginx/html
COPY --from=builder /app/public/url.json /usr/share/nginx/html/url.json

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
