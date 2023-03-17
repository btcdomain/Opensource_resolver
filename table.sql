create database if not exists domain_inscription_data;
use domain_inscription_data;

create table domain_inscription_info(
    id bigint primary key auto_increment,
    inscribe_num bigint,
    inscribe_id varchar(200),
    sat bigint,
    domain_name varchar(200),
    address varchar(200),
    create_time bigint,
    update_time bigint
);

