
# Setup Instructions

Follow these steps to set up and run the Todo App:

## Prerequisites
- Install [Docker](https://www.docker.com/).
- Install [Docker Compose](https://docs.docker.com/compose/install/).

## Environment Configuration
1. Copy the `.env.example` file to `.env`:

```bash
cp .env.example .env
```

2. Push the schema from the `schema.sql` file
3. update the sqlx cargo cache:
```bash
cargo sqlx prepare
```
4. Run the rust binary:
```bash
cargo run
```