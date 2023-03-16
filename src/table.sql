create database inscribe_data;
use inscribe_data;

create table inscribe_info(
    id bigint primary key auto_increment,
    inscribe_num bigint,
    inscribe_id varchar(200),
    sat bigint,
    domain_name varchar(200),
    address varchar(200),
    create_time bigint,
    update_time bigint
);

create table domain_info(
    id bigint primary key auto_increment,
    domain_name varchar(200),
    owner_address varchar(200),
    img_url varchar(200)
);
