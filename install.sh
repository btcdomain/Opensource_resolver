image_name='btcdomain_resolver_mysql'
pwd='btcdomain'
port=3309

docker pull mysql:latest

docker run -itd --name ${image_name} -p $port:$port -v table.sql:/docker-entrypoint-initdb.d/table.sql -e MYSQL_ROOT_PASSWORD=btcdomain mysql

docker start ${image_name}

export RUST_LOG=info
export database=mysql://root:$pwd@localhost:$port/domain_inscription_data

wget https://github.com/btcdomain/btcdomain_resolver/releases/download/v0.1.3/btcdomain_resolver
     
chmod +x btcdomain_resolver

nohup ./btcdomain_resolver > btcdomain_resolver.log &