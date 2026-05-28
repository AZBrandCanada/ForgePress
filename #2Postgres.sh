#!/bin/bash
set -e

echo "=========================================================="
echo "    Installing and Configuring PostgreSQL on Fedora       "
echo "=========================================================="

# 1. Install PostgreSQL Packages via DNF
echo "Installing PostgreSQL packages..."
sudo dnf install -y postgresql-server postgresql-contrib

# 2. Initialize PostgreSQL Database (Fedora Specific Requirement)
if [ ! -f /var/lib/pgsql/data/PG_VERSION ]; then
    echo "Initializing PostgreSQL database storage..."
    sudo postgresql-setup --initdb
else
    echo "PostgreSQL database storage is already initialized."
fi

# 3. Configure Local Password Authentication (MD5)
# Fedora defaults to 'peer' or 'ident' auth, which blocks password connection strings.
# We replace these with 'md5' so the Rust string "postgres://forge_user:forge_pass@..." is accepted.
echo "Configuring authentication permissions (switching peer/ident to md5)..."


# 4. Enable and Start the Systemd Service
echo "Enabling and starting PostgreSQL service..."
sudo systemctl enable --now postgresql

# 5. Create Database User and Schema Matching .env.example
echo "Creating user 'forge_user' and database 'forgepress_db'..."

# Create user if not exists
sudo -u postgres psql -tAc "SELECT 1 FROM pg_roles WHERE rolname='forge_user'" | grep -q 1 || \
sudo -u postgres psql -c "CREATE USER forge_user WITH PASSWORD 'forge_pass';"

# Create database owned by user if not exists
sudo -u postgres psql -tAc "SELECT 1 FROM pg_database WHERE datname='forgepress_db'" | grep -q 1 || \
sudo -u postgres psql -c "CREATE DATABASE forgepress_db OWNER forge_user;"

# Grant all schemas access (PostgreSQL 15+ permission restriction security safety)
sudo -u postgres psql -d forgepress_db -c "GRANT ALL ON SCHEMA public TO forge_user;"
sudo sed -i 's/ident/md5/g' /var/lib/pgsql/data/pg_hba.conf
sudo sed -i 's/peer/md5/g' /var/lib/pgsql/data/pg_hba.conf
# 6. Restart Service to Apply Auth Configuration
echo "Restarting PostgreSQL to apply configurations..."
sudo systemctl restart postgresql

echo "=========================================================="
echo " Success! PostgreSQL is fully installed, configured,      "
echo " and running.                                             "
echo "                                                          "
echo " You can now run `./dev.sh` to start developing!          "
echo "=========================================================="