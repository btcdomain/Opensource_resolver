use std::{time::{Duration, Instant}, sync::{Arc, Mutex, mpsc, RwLock}, thread, collections::VecDeque};
use log::{info,warn};
use crate::*;

const FATAL_NOLOCK: &str = "error acquiring task lock";
const FATAL_RCVTSK: &str = "error receiving task";

lazy_static::lazy_static! {
    pub static ref SYNC_DATA: Arc<RwLock<VecDeque<String>>> = Arc::new(RwLock::new(VecDeque::new()));
    pub static ref CUR_NUMBER: Arc<RwLock<VecDeque<u64>>> = Arc::new(RwLock::new(VecDeque::new()));
}

pub async fn sched_work() {
    info!("sched_work start");
    let _ = crossbeam::thread::scope(|s| {
        let sync_handle = s.spawn(|_| {
            sync_data_task(1, 30);
        });

        let update_handle = s.spawn(|_| {
            update_task(1, 3600 * 2);
        });

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
        sync_data_tx.clone().send(q).expect("sync workd send error");
        thread::sleep(Duration::from_secs(interval));
    }
}

fn sync_data_task_inner() {
    info!("start sync_data_task, {:?}", &(*SYNC_DATA).read().unwrap().len());
    if &(*SYNC_DATA).read().unwrap().len() == &0 {
        let _ =&(*SYNC_DATA).write().unwrap().push_back(String::from("sync data"));
        let lastest = query_lastest_number();
        let mut max_number = if &(*CUR_NUMBER).read().unwrap().len() == &0 {
            std::cmp::max(lastest, START_INSCRIPTION_NUMBER)
        }else {
            let cur_number = &(*CUR_NUMBER).write().unwrap().pop_front().unwrap();
            info!("cur_number: {}, lastest: {}", cur_number, lastest);
            std::cmp::max(*cur_number - 30, lastest)
        };
        
        let mut break_count = 0;
        
        loop {
            max_number += 1;
            info!("query number: {}", max_number);
            let (inscribe_result, _) = get_inscribe_by_number(max_number);
            // info!("inscribe_result: {:?}", inscribe_result);
            if inscribe_result.is_some() {
                let content = inscribe_result.unwrap();
                let content_data = content.content;
                let inscribe_num = content.inscribe_num;
                let inscribe_id = content.inscribe_id;
                let address = content.address;
                let length = content_data.len();
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
                        if ecdsa::verify(&sign_data, &inscribe_data.sig) {
                            info!("ecds signature verify success");
                            let info = InscribeInfo { 
                                id: 0,
                                inscribe_num: inscribe_num, 
                                inscribe_id: inscribe_id, 
                                sat: 0, 
                                domain_name: domain_name.clone(), 
                                address: address,
                                create_time: get_now_time(),
                                update_time: get_now_time(),
                                expire_date: expire_date,
                                register_date: inscribe_data.register_date,
                            };
                            let insert_result = insert_inscribe_info(info);
                            info!("insert_result: {:?}", insert_result);
                            if insert_result.is_ok() {
                                
                            }else {
                                break;
                            }
                        }else {
                            info!("ecds signature verify failed");
                            continue;
                        }
                        
                    }else {

                    }

                }

            }else {
                break_count += 1;
                if break_count > 20 {
                    break;
                }
            }
            
        }
        let _ = &(*CUR_NUMBER).write().unwrap().push_back(max_number);
        let _ = &(*SYNC_DATA).write().unwrap().pop_front().unwrap();
    }else {
        info!("Syncing data!");
    }
    
    
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
        update_tx.clone().send(q).expect("sync update task send error");
        thread::sleep(Duration::from_secs(interval));
    }
}

fn update_task_inner() {
    info!("start update_task, {:?}", &(*SYNC_DATA).read().unwrap().len());
    if &(*SYNC_DATA).read().unwrap().len() == &0 {
        let _ =&(*SYNC_DATA).write().unwrap().push_back(String::from("update"));
        let all_domains = query_all();
        for info in all_domains.iter() {
            let (inscribe_result, _) = get_inscribe_by_number(info.inscribe_num);
            if inscribe_result.is_some() {
                let content = inscribe_result.unwrap();
                let content_data = content.content;
                
                let address = content.address;
                let length = content_data.len();
                if length > 350 && length < 500 {
                    let format_data = serde_json::from_slice(&content_data);
                    if format_data.is_ok() {
                        let inscribe_data: InscribeData = format_data.unwrap();
                        // info!("inscribe data: {:?}", inscribe_data);
                        
                        let domain_name = inscribe_data.name;
                        let expire_date = inscribe_data.expire_date;
                        let now_date = get_now_time();
                        if expire_date < now_date {
                            warn!("domain: {}, is expired, now: {}, expire_time: {}", domain_name, now_date, expire_date);
                            let delete_result = delete_from_id(info.id);
                            info!("delete_result: {:?}, domain: {}", delete_result, &domain_name);
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
                        if ecdsa::verify(&sign_data, &inscribe_data.sig) {
                            info!("ecds signature verify success");
                            let update_result = update_inscribe_info(info.id, &address);
                            if update_result.is_ok() {
                                
                            }else {
                                break;
                            }
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
        let _ = &(*SYNC_DATA).write().unwrap().pop_front().unwrap();
    }else {
        info!("Updating data!");
    }
    
}
