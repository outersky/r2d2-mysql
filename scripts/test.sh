#!/usr/bin/env bash

DATABASE_NAME="rust_mysql_test"
export DATABASE_URL="mysql://root@localhost:3306/${DATABASE_NAME}"

# Assume no root password and create database
mysql -uroot -e "CREATE DATABASE ${DATABASE_NAME}"

# Run the tests!
cargo test

# Clean up after ourselves and drop the database
mysql -uroot -e "DROP DATABASE ${DATABASE_NAME}"

