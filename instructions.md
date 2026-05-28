here is a quick, practical checklist to
make sure your first execution runs smoothly.

1. Verify Your Environment

Ensure you have the following runtimes installed locally on your system:

  - Rust: Run rustc --version to make sure you are on a stable, up-to-date
    version.
  - Node.js & NPM: Run node --version and npm --version (required to bundle the
    static admin dashboard).
  - Wasm Target (Optional): If you plan to compile WebAssembly plugins later,
    run:
    rustup target add wasm32-wasip2

2. Populate the Directory Structure

If you haven't already, use the setup script (setup.sh or setup.ps1 provided
earlier) to automatically generate all the subdirectories and empty files. Then,
copy and paste the code we implemented into those files.

3. Compile the Workspace

Run the automated compiler script to build the frontend and compile the Rust
workspace:

# Make the script executable
chmod +x compile.sh

# Run the compilation
./compile.sh

4. Create the Configuration File

Copy the example environment template to create your live configuration:

cp .env.example .env

(By default, this is configured to use a local SQLite file database at
content/forgepress.db. If you want to use PostgreSQL, make sure Postgres is
running and update the DATABASE_URL string in the .env file.)

5. Register Your Admin User

Use the compiled CLI tool to securely hash and write your first administrator
account directly into the database:

./target/release/forgepress-cli create-admin --username admin --email admin@yoursite.com --password mysecurepassword

6. Scaffold Your Default Theme

Use the CLI to automatically build the default layout files under your active
theme:

./target/release/forgepress-cli install-theme --name default

7. Launch the Engine

With the user registered and the theme scaffolded, boot up the main server:

./target/release/forgepress-core

Your system is now fully live and running. You can navigate to
http://localhost:8080/api/admin in your browser, log in using the credentials
from Step 5, and start designing your dynamic layouts block-by-block.


