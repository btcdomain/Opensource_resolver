create database if not exists domain_inscription_data;
use domain_inscription_data;

create table if not exists domain_inscription_info(
    id bigint primary key auto_increment,
    inscribe_num bigint,
    inscribe_id varchar(200),
    sat bigint,
    domain_name varchar(200),
    address varchar(200),
    create_time bigint,
    update_time bigint,
    expire_date bigint,
    register_date bigint
);

-- INSERT INTO domain_inscription_data.domain_inscription_info VALUE(0, 352916, '01a22903bf8ba76d68edd1d1cd344178591713ffc7ce718a12704e1135da5126i0', 0, 'free.btc', 'bc1px3hey79zhn87vkj7y4hgmkzu3glzqnzhu6fawm6ape5p89l4n77qcy8t0m', 1678280074069, 1678280074069, );