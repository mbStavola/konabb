FROM node:alpine

EXPOSE 3000

WORKDIR /usr/src/konabb

# Install dependencies
COPY package.json ./
COPY package-lock.json ./
RUN npm install

# Copy Source
COPY server.js ./
COPY components ./components
COPY pages ./pages
COPY utils ./utils

# Build site
RUN npm run build

ENTRYPOINT [ "npm", "run", "start" ]
