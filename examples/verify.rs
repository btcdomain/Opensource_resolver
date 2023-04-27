use btcdomain_resolver::{BtcDomainLinkSign, verify_compact};

fn main () {
    let _type = String::from("btcdomain_link");
    let domain = String::from("helloworld.btc");
    let ins_id = String::from("c0ff5c133d424706ca76c4f39f98a0f876b8e04fdf0fde5b5a0934252342da68i0");
    let public_key = String::from("0369a4b3cbbe8959544db9cc3bcf8ced465d27daea08ff6c4558e7f1890507cd9a");
    let sig = String::from("f34cc5174ee23b741f13845f105757a68418209809aa21f9d2fa28f0ac0774f71e716cbf50c57f6f6c699f79e20b1544d3e5f49c021005800980452f42f22c5d");
    let data = BtcDomainLinkSign {
        _type: _type.clone(),
        domain: domain.clone(),
        obj_ins_id: ins_id.clone(),
        public_key: public_key.clone()
    };


    let sign_data = serde_json::to_vec(&data).unwrap();
    let result = verify_compact(&sign_data, &sig, &public_key);
    println!("verify result: {}", result);
}
