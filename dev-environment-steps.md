# Dev Env Build Steps

Once you have forked the repository and commited your fix/feature addon, follow the below steps to create a dev environment and test your code.

1. Pull Github Repo

    ``` bash
    git clone https://github.com/<UserID>/netvisor.git
    cd netvisor
    ```

2. Install NVM

    ``` bash
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
    nvm install 20
    nvm use 20
    ```

3. Install postgresql-17

    ``` bash
    sudo apt install curl ca-certificates gnupg2 wget vim -y
    sudo install -d /usr/share/postgresql-common/pgdg
    sudo curl -o /usr/share/postgresql-common/pgdg/apt.postgresql.org.asc --fail https://www.postgresql.org/media/keys/ACCC4CF8.asc
    sudo sh -c 'echo "deb [arch=amd64 signed-by=/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc] https://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
    sudo sh -c 'echo "deb [arch=amd64 signed-by=/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc] https://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
    sudo apt update
    sudo apt -y install postgresql-17
    ```

4. Run `make` commands

    ``` bash
    make install-dev-linux
    make dev-server
    make dev-ui
    make dev-daemon
    ```

5. Access Dev UI via 'http:\\<IP>:5173'
