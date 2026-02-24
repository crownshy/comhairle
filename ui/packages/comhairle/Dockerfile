# ---- Base Stage ----
FROM node:22 AS base
WORKDIR /app

# Install pnpm globally
RUN corepack enable && corepack prepare pnpm@9.15.0 --activate

# Copy package files and install dependencies
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

# ---- Build Stage ----
FROM base AS build

ENV NODE_OPTIONS="--max-old-space-size=4096"

WORKDIR /app

# Copy all project files
COPY . .

# Build the SvelteKit app
RUN pnpm run build

# ---- Production Stage ----
FROM node:22-slim AS production
ENV NODE_OPTIONS="--max-old-space-size=4096"
WORKDIR /app

# Install pnpm globally in the final image
RUN corepack enable && corepack prepare pnpm@9.15.0 --activate

# Copy built files and only necessary dependencies
COPY --from=build /app/build ./build
COPY --from=build /app/package.json ./
COPY --from=build /app/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile --prod

# Set environment variables
ENV NODE_ENV=production
ENV PORT=3000

# Expose the application port
EXPOSE 3000

# Run the application
CMD ["node", "build"]
