use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use std::{time::{Duration}, sync::{Arc, Mutex, mpsc, RwLock}, thread, collections::VecDeque};
use rocket::log::{info_ as info, warn_ as warn};
use crate::{get_now_time, InscribeSignData, START_INSCRIPTION_NUMBER, InscribeData, verify, repo::DomainInscriptionInfo, 
    get_inscribe_by_number, PUBLIC_KEY, ord_index_service, get_content_cmd, get_addr_by_id, get_content_by_id_api, get_content_by_number_api};

lazy_static! {
    pub static ref SYNC_DATA: Arc<RwLock<VecDeque<String>>> = Arc::new(RwLock::new(VecDeque::new()));
    pub static ref CUR_NUMBER: Arc<RwLock<VecDeque<i64>>> = Arc::new(RwLock::new(VecDeque::new()));
    pub static ref ORD_INDEX: Arc<RwLock<VecDeque<String>>> = Arc::new(RwLock::new(VecDeque::new()));

}

const FATAL_NOLOCK: &str = "error acquiring task lock";
const FATAL_RCVTSK: &str = "error receiving task";

pub async fn sched_work() {
    info!("sched_work start");
    let _ = &(*ORD_INDEX).write().unwrap().push_back(String::from("ord_index"));

    let _ = crossbeam::thread::scope(|s| {
        let sync_handle = s.spawn(|_| {
            sync_data_task(1, 60);
        });

        let update_handle = s.spawn(|_| {
            update_task(1, 600);
        });

        let index_handle = s.spawn(|_| {
            index_task(1, 30);
        });

        index_handle.join().unwrap();

        sync_handle.join().unwrap();
        update_handle.join().unwrap();
    });
}

pub enum SyncDataInput{
    SyncData(),
    Shutdown,
}
pub struct SyncDataWorker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>
}
impl SyncDataWorker {
    pub fn start (
        id: usize,
        sync_data_tx: Arc<Mutex<mpsc::Receiver<SyncDataInput>>>,
     ) -> SyncDataWorker {
        let thread = thread::spawn(move || loop {
            let task = {
                let rx = sync_data_tx.lock().expect(FATAL_NOLOCK);
                rx.recv().expect(FATAL_RCVTSK)
            };

            match task {
                SyncDataInput::SyncData() => {
                    sync_data_task_inner();
                },
                SyncDataInput::Shutdown => break,
            }
        });

        SyncDataWorker { id: id, thread: Some(thread) }
     }

}

pub fn sync_data_task(sync_num_worker: usize, interval: u64){
    let (sync_data_tx, _) = {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let workers: Vec<SyncDataWorker> = (0..sync_num_worker).map(|n| {
            SyncDataWorker::start(n as usize, rx.clone())
        }).collect();
        (tx, workers)
    };
    loop {
        let q = SyncDataInput::SyncData();
        sync_data_tx.clone().send(q).ok();
        thread::sleep(Duration::from_secs(interval));
    }
}

fn sync_data_task_inner() {
    info!("start sync_data_task, {:?}", &(*SYNC_DATA).read().unwrap().len());
    // if &(*SYNC_DATA).read().unwrap().len() == &0 {
    //     let _ =&(*SYNC_DATA).write().unwrap().push_back(String::from("sync data"));
        let lastest = DomainInscriptionInfo::query_lastest_number().unwrap();
        let mut max_number = if &(*CUR_NUMBER).read().unwrap().len() == &0 {
            std::cmp::max(lastest, START_INSCRIPTION_NUMBER)
        }else {
            let cur_number = &(*CUR_NUMBER).write().unwrap().pop_front().unwrap();
            info!("cur_number: {}, lastest: {}", cur_number, lastest);
            std::cmp::max(*cur_number - 30, lastest)
        };
        
        let mut break_count = 0;
        let mut counter = 0;
        loop {
            counter += 1;
            if counter == 2000 {
                break;
            }
            max_number += 1;
            info!("query number: {}", max_number);
            let inscribe_result = Runtime::new().unwrap().block_on(get_content_by_number_api(max_number));
            if inscribe_result.is_ok() {
                let content = inscribe_result.unwrap();
                let content_data = content.content;
                let inscribe_num = content.inscribe_num;
                let inscribe_id = content.inscribe_id;
                let length = content_data.len();
                let address = content.address;
                if length > 350 && length < 500 {
                    let format_data = serde_json::from_slice(&content_data);
                    if format_data.is_ok() {
                        let inscribe_data: InscribeData = format_data.unwrap();
                        info!("inscribe data: {:?}", inscribe_data);
                        
                        let domain_name = inscribe_data.name;
                        let expire_date = inscribe_data.expire_date;
                        let now_date = get_now_time();
                        if expire_date < now_date {
                            warn!("domain: {}, is expired, now: {}, expire_time: {}", domain_name, now_date, expire_date);
                            continue;
                        }

                        let sign_info = InscribeSignData{
                            name: domain_name.clone(),
                            first_owner: inscribe_data.first_owner,
                            create_date: inscribe_data.create_date,
                            register_date: inscribe_data.register_date,
                            expire_date: expire_date
                        };
                        let sign_data = serde_json::to_vec(&sign_info).unwrap();


                        if verify(&sign_data, &inscribe_data.sig, PUBLIC_KEY) {
                            info!("ecds signature verify success");
                            let info = DomainInscriptionInfo { 
                                id: 0,
                                inscribe_num: inscribe_num as i64, 
                                inscribe_id: inscribe_id, 
                                sat: 0, 
                                domain_name: domain_name.clone(), 
                                address: address,
                                create_time: get_now_time(),
                                update_time: get_now_time(),
                                expire_date: expire_date,
                                register_date: inscribe_data.register_date,
                            };
                            let check = DomainInscriptionInfo::query_by_domain(&domain_name);
                            if check.is_ok() {

                            }else {
                                let insert_result = DomainInscriptionInfo::insert_inscribe_info(info);
                                info!("insert_result: {:?}", insert_result);
                            }
                        }else {
                            info!("ecds signature verify failed");
                            continue;
                        }
                        
                    }else {

                    }

                }

            }else {
                max_number -= 1;
                break_count += 1;
                if break_count > 10 {
                    break;
                }
            }
            
        }
        let _ = &(*CUR_NUMBER).write().unwrap().push_front(max_number);
    //     let _ = &(*SYNC_DATA).write().unwrap().pop_front().unwrap();
    // }else {
    //     info!("Syncing data!");
    // }
    
    
}


pub enum UpdateInput{
    Update(),
    Shutdown,
}
pub struct UpdateWorker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>
}
impl UpdateWorker {
    pub fn start (
        id: usize,
        update_tx: Arc<Mutex<mpsc::Receiver<UpdateInput>>>,
     ) -> UpdateWorker {
        let thread = thread::spawn(move || loop {
            let task = {
                let rx = update_tx.lock().expect(FATAL_NOLOCK);
                rx.recv().expect(FATAL_RCVTSK)
            };

            match task {
                UpdateInput::Update() => {
                    update_task_inner();
                },
                UpdateInput::Shutdown => break,
            }
        });

        UpdateWorker { id: id, thread: Some(thread) }
     }

}

pub fn update_task(sync_num_worker: usize, interval: u64){
    let (update_tx, _) = {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let workers: Vec<UpdateWorker> = (0..sync_num_worker).map(|n| {
            UpdateWorker::start(n as usize, rx.clone())
        }).collect();
        (tx, workers)
    };
    loop {
        let q = UpdateInput::Update();
        update_tx.clone().send(q).ok();
        thread::sleep(Duration::from_secs(interval));
    }
}

fn update_task_inner() {
    info!("start update_task, {:?}", &(*SYNC_DATA).read().unwrap().len());
    // if &(*SYNC_DATA).read().unwrap().len() == &0 {
    //     let _ =&(*SYNC_DATA).write().unwrap().push_back(String::from("update"));
        let all_domains = DomainInscriptionInfo::query_all().unwrap();
        let total_size = all_domains.len();
        for (idx, info) in all_domains.iter().enumerate() {
            if idx % 100 == 0 {
                info!("[update taks] idx: {}, total: {}", idx, total_size);
            }
            let inscribe_result = Runtime::new().unwrap().block_on(get_content_by_id_api(&info.inscribe_id));
            if inscribe_result.is_ok() {
                let content = inscribe_result.unwrap();
                let content_data = content.content;
                let address = content.address;
                let length = content_data.len();
                if length > 350 && length < 500 {
                    let format_data = serde_json::from_slice(&content_data);
                    if format_data.is_ok() {
                        let inscribe_data: InscribeData = format_data.unwrap();
                        
                        let domain_name = inscribe_data.name;
                        let expire_date = inscribe_data.expire_date;
                        let now_date = get_now_time();
                        if expire_date < now_date {
                            warn!("domain: {}, is expired, now: {}, expire_time: {}", domain_name, now_date, expire_date);
                            let delete_result = DomainInscriptionInfo::delete_info(info.id);
                            warn!("delete_result: {:?}, domain: {}, is expired, now: {}, expire_time: {}", delete_result, &domain_name, now_date, expire_date);
                            continue;
                        }

                        if address == info.address {
                            continue;
                        }

                        let sign_info = InscribeSignData{
                            name: domain_name.clone(),
                            first_owner: inscribe_data.first_owner,
                            create_date: inscribe_data.create_date,
                            register_date: inscribe_data.register_date,
                            expire_date: inscribe_data.expire_date
                        };
                        
                        let sign_data = serde_json::to_vec(&sign_info).unwrap();
                        if verify(&sign_data, &inscribe_data.sig, PUBLIC_KEY) {
                            info!("[update]ecds signature verify success, number: {}, domain: {}, address: {}, new_address: {}", info.inscribe_num, info.domain_name, info.address, address);
                            let update_result = DomainInscriptionInfo::update_info_address(info.id, &address);
                            info!("update_result: {:?}", update_result);
                            
                        }else {
                            warn!("ecds signature verify failed");
                            continue;
                        }
                        
                    }else {

                    }

                }

            }else {
                break;
            }
            
        }
        // let _ = &(*SYNC_DATA).write().unwrap().pop_front().unwrap();
    // }else {
    //     info!("Updating data!");
    // }
    
}

enum IndexInput{
    Index(),
    _Shutdown,
}
struct IndexWorker {
    pub _id: usize,
    pub _thread: Option<thread::JoinHandle<()>>
}
impl IndexWorker {
    pub fn start (
        id: usize,
        query_tx: Arc<Mutex<mpsc::Receiver<IndexInput>>>,
     ) -> IndexWorker {
        let thread = thread::spawn(move || loop {
            let task = {
                let rx = query_tx.lock().expect(FATAL_NOLOCK);
                rx.recv().expect(FATAL_RCVTSK)
            };

            match task {
                IndexInput::Index() => {
                    index_inner();
                },
                IndexInput::_Shutdown => break,
            }
        });

        IndexWorker { _id: id, _thread: Some(thread) }
     }

}

fn index_task(query_num_worker: usize, interval: u64){
    let (clear_tx, _) = {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let workers: Vec<IndexWorker> = (0..query_num_worker).map(|n| {
            IndexWorker::start(n as usize, rx.clone())
        }).collect();
        (tx, workers)
    };
    loop {
        let q = IndexInput::Index();
        clear_tx.clone().send(q).expect("ord index send error");
        thread::sleep(Duration::from_secs(interval));
    }
}

fn index_inner() {
    info!("ord_index_inner: {:?}", &(*ORD_INDEX).read().unwrap().len());
    if &(*ORD_INDEX).read().unwrap().len() > &0 {
        info!("There are ((({:?}))) domain INDEX_inner", &(*ORD_INDEX).read().unwrap().len());
        ord_index_service();       
    }
}